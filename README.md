# 一个精简的容器运行时 😍

## 理论部分

### 容器是一个进程
>  it’s a forked or cloned process
如果用一个词语描述的话，容器就是一个clone出的进程

- 有自己的pid
- 被一个user或者group拥有
- 能够通过ps命令被列出，
- 能够向它发出signal

### 容器如何与操作系统隔离
**答案: 命名空间**
> Namespaces provide the logical isolation of resources for processes running in different sets of namespaces
命名空间为运行在**不同的命名空间的进程**提供**在逻辑上互相隔离**的资源, 命名空间有以下几种
- MOUNT namespace: 当前容器能看到的所有挂载点 `for all mount points that the current process can see`
    mount命名空间包含进程可以看到的挂载点列表(文件列表)，左右的挂载点都能从父命名空间copy到子命名空间，但反过来不行，因此子进程单独创建的挂载点只有子节点能看到，当挂载点被取消后，也不会影响父进程
    `The MOUNT namespace contains the list of mount points a process can see. When first cloning from a mount namespace (the CLONE_NEWNS flag) all mount points are copied from the parent to the child namespace. Any additional mount point created in the child isn’t propagated to the parent mount namespace. Also, when the child process unmounts any mount point, it’s only being affected inside his mount namespace.`
- NETWORK namespaces: 网络接口和流量相关 `for the network interfaces and traffic rules`

    单独的network命名空间拥有独立的网络堆栈、路由表、防火墙和环回接口`A separate NETWORK namespace gets its own network stack, routing table, firewalls and a loopback interface.`
    绑定到各自环回设备的具有不同网络命名空间的两个进程会被绑定到单独的逻辑接口，以便它们之间流量不会产生干扰.
    `Two processes with different network namespaces that bind to their respective loopback devices are bound to a separate logical interface so that traffic doesn’t interfere between them.`
- PID namespace: 进程树的pid `for the process tree`

    两个运行在不同pid的进程互相看不到对方 ` Two processes running in different PID namespaces don’t see the same process tree.`
- .. and so on.

下图是3个独立的命名空间的例子

![](https://trdthg-img-for-md-1306147581.cos.ap-beijing.myqcloud.com/img/202202271707484.png)

### 查看命名空间

每一个容器一般都有一个根挂载点`/`, 根挂载点一般不是同一个目录，docker会为每个容器的根挂载点创建一个单独的目录，这样容器的文件系统就与其他容器区分开了

每个进程都在主机上有一个文件夹`/proc/{PID}/ns`与之对应

这里是第一个shell，用jobs查看，当前有4个运行在后台的sleep进程
```shell
»»»» jobs
Job	Group	CPU	State	Command
4	337762	0%	running	sleep 1001 &
3	337096	0%	running	sleep 1000 &
2	337061	0%	running	sleep 1000 &
1	336406	0%	running	sleep 1000 &
```
查看job4 job3的命名空间，结果发现他们具有相同的命名空间符号链接
```shell
»»»» ls -la 337762/ns
total 0
dr-x--x--x 2 trdthg trdthg 0 Feb 27 17:36 .
dr-xr-xr-x 9 trdthg trdthg 0 Feb 27 17:31 ..
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 cgroup -> 'cgroup:[4026531835]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 ipc -> 'ipc:[4026531839]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 mnt -> 'mnt:[4026531840]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 net -> 'net:[4026531992]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 pid -> 'pid:[4026531836]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 pid_for_children -> 'pid:[4026531836]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 time -> 'time:[4026531834]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 time_for_children -> 'time:[4026531834]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 user -> 'user:[4026531837]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 uts -> 'uts:[4026531838]'
```
```shell
»»»» ls -la 337096/ns
total 0
dr-x--x--x 2 trdthg trdthg 0 Feb 27 17:36 .
dr-xr-xr-x 9 trdthg trdthg 0 Feb 27 17:27 ..
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 cgroup -> 'cgroup:[4026531835]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 ipc -> 'ipc:[4026531839]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 mnt -> 'mnt:[4026531840]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 net -> 'net:[4026531992]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 pid -> 'pid:[4026531836]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 pid_for_children -> 'pid:[4026531836]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 time -> 'time:[4026531834]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 time_for_children -> 'time:[4026531834]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 user -> 'user:[4026531837]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:36 uts -> 'uts:[4026531838]'
```
这里是第二个shell，查看发现一样
```
»»»» jobs
Job	Group	CPU	State	Command
1	338498	0%	running	sleep 100 &
»»»» ls -la 338498/ns
total 0
dr-x--x--x 2 trdthg trdthg 0 Feb 27 17:37 .
dr-xr-xr-x 9 trdthg trdthg 0 Feb 27 17:35 ..
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 cgroup -> 'cgroup:[4026531835]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 ipc -> 'ipc:[4026531839]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 mnt -> 'mnt:[4026531840]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 net -> 'net:[4026531992]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 pid -> 'pid:[4026531836]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 pid_for_children -> 'pid:[4026531836]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 time -> 'time:[4026531834]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 time_for_children -> 'time:[4026531834]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 user -> 'user:[4026531837]'
lrwxrwxrwx 1 trdthg trdthg 0 Feb 27 17:37 uts -> 'uts:[4026531838]'
```
结论就是在没有任何限制的情况下运行ssleep命令，即使shell不同，他们的命名空间也是相同的，他们属于相同的命名空间

### 创造不同的命名空间
#### UNSHARE
`linux`提供了`sched.h`库的`UNSHARE`系统调用，这个syscall能够更改进程运行时的context，将其从根命名空间中分离出去，从而创建新的它自己的命名空间，但是仅仅有`UNSHARE`还不够(比如，从根PID分离出去的sleep子进程都会进入同一个新的命名空间)，所以通常需要在容器运行时调用`UNSHARE`
之后然后是一个`fork/vfork`调用来创建实际的容器进程。

#### CLONE
clone主要用来创建新的命名空间：该系统调用和`UNSHARE`一样能够指定命名空间，然后派生出子进程并为子进程创建堆栈

#### SETNS(对应nsetns命令)
让当前线程加入一个命名空间：通过文件描述符将其命名空间修改为一个已经存在的命名空间，例如

nsetns是对setns做的封装，不需要fd，指定pid即可
```
// fock一个shell，并写这个shell关联的是PID 15对应的命名空间
// 说人话就是: 在/proc/15/ns/mnt的命名空间内执行/bin/bash
nsetns --mount=/proc/15/ns/mnt /bin/bash
```

### 使用nsetns访问docker容器
1. 使用docker创建一个alpine容器，找到pid，并查看它的命名空间，可以看到有一些命名空间是相同的(虽然大部分都不同)
```shell
»»»» sudo docker run -d --rm alpine sleep 1000;
bf68ba8e9d7b0d83e10c960c2b273b57444f354a9fbf1589f121cf0e3d246d9d
»»»» ps -aux | grep sleep
root      345747  0.2  0.0   1584     4 ?        Ss   18:28   0:00 sleep 1000
trdthg    345850  0.0  0.0  10076  2532 pts/7    S+   18:28   0:00 grep --color=auto sleep
»»»» sudo ls -la /proc/345747/ns
total 0
dr-x--x--x 2 root root 0 Feb 27 18:28 .
dr-xr-xr-x 9 root root 0 Feb 27 18:28 ..
lrwxrwxrwx 1 root root 0 Feb 27 18:29 cgroup -> 'cgroup:[4026533097]'
lrwxrwxrwx 1 root root 0 Feb 27 18:29 ipc -> 'ipc:[4026533037]'
lrwxrwxrwx 1 root root 0 Feb 27 18:29 mnt -> 'mnt:[4026533035]'
lrwxrwxrwx 1 root root 0 Feb 27 18:28 net -> 'net:[4026533040]'
lrwxrwxrwx 1 root root 0 Feb 27 18:29 pid -> 'pid:[4026533038]'
lrwxrwxrwx 1 root root 0 Feb 27 18:29 pid_for_children -> 'pid:[4026533038]'
lrwxrwxrwx 1 root root 0 Feb 27 18:29 time -> 'time:[4026531834]'
lrwxrwxrwx 1 root root 0 Feb 27 18:29 time_for_children -> 'time:[4026531834]'
lrwxrwxrwx 1 root root 0 Feb 27 18:29 user -> 'user:[4026531837]'
lrwxrwxrwx 1 root root 0 Feb 27 18:29 uts -> 'uts:[4026533036]'
```
2. 利用nsetns访问
只需要利用PID即可
```
»»»» sudo nsenter --mount=/proc/345747/ns/mnt /bin/ash
/ # ls
```

我们在容器的命名空间内运行了一个shell进程，容器的根命名空间和主机的不同，所以
```
docker exec -it <CONTAINER_ID> <CMD>
```
等于
```
nsenter -a -t <CONTAINER_PID> <CMD>
```

### docker做了什么
经过上面的讨论，`docker run`命令会为容器fock一个进程，更具体一点就是docker(其实是containerd(守护进程))会调用底层的容器运行时(runc)创建一个指定的命名空间，准备容器环境，并在用户定义的命令发生前执行一些特殊命令

docker本身只是做了大量配置工作(管理config.json, 容器根目录等)，拉取镜像，管理网络等工作

### OCI规范
> An OCI-compliant container runtime is a CLI binary that implements the following commands:
符合OCI规范的容器运行时是一个实现了以下cli命令的二进制文件
``` rs
create <id> <bundle_path>
start <id>
state <id>
kill <id> <signal>
delete <id>
```

## rust实现部分 todo

## 附录

### nsetns命令
```
nsenter [options] [program [arguments]]

options:
-t, --target pid：指定被进入命名空间的目标进程的pid
-m, --mount[=file]：进入mount命令空间。如果指定了file，则进入file的命令空间
-u, --uts[=file]：进入uts命令空间。如果指定了file，则进入file的命令空间
-i, --ipc[=file]：进入ipc命令空间。如果指定了file，则进入file的命令空间
-n, --net[=file]：进入net命令空间。如果指定了file，则进入file的命令空间
-p, --pid[=file]：进入pid命令空间。如果指定了file，则进入file的命令空间
-U, --user[=file]：进入user命令空间。如果指定了file，则进入file的命令空间
-G, --setgid gid：设置运行程序的gid
-S, --setuid uid：设置运行程序的uid
-r, --root[=directory]：设置根目录
-w, --wd[=directory]：设置工作目录
```

### 参考
- [Container Runtime in Rust — Part 0](https://itnext.io/container-runtime-in-rust-part-0-7af709415cda)
- [云原生CTO公众号中文翻译]()
- [nsenter命令简介](https://staight.github.io/2019/09/23/nsenter%E5%91%BD%E4%BB%A4%E7%AE%80%E4%BB%8B/)
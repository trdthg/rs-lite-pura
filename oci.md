# OCI规范

## 运行时规范

### 容器格式
一个标准的container bundle包含加载和运行容器所需的所有信息,包括
1. `config.json`: 包含配置数据


2. `rootfs`: 容器的根文件系统

    `config.json`中的`route.path`字段指定,默认为`rootfs`

### 运行时和生命周期

#### 状态
包含以下属性
- ociVersion(string, REQUIRED): 遵守的OCI运行时规范规范的版本。
- id(string, REQUIRED): 是容器的ID. 这在此主机上的所有容器中必须是唯一的. 不要求它在主机之间是唯一的。
- status(string, REQUIRED): 是容器的运行时状态。该值可以是以下之一：
    - creating：正在创建容器（生命周期中的第 2 步）
    - created: 运行时已经完成创建操作（生命周期第2步之后）,容器进程既没有退出也没有执行用户指定的程序
    - running: 容器进程已经执行了用户指定的程序但还没有退出（在生命周期的第 8 步之后）
    - stopped：容器进程已退出（生命周期中的第 10 步）
    附加值可以由运行时定义,但是,它们必须用于表示上面未定义的新运行时状态。

- pid(int, REQUIRED when status is created or running on Linux, OPTIONAL 在其他平台) 是容器进程的ID。对于在运行时命名空间中执行的钩子,它是运行时看到的 pid。对于在容器命名空间中执行的钩子,它是容器看到的 pid。

- bundle(string, REQUIRED) 是容器包目录的绝对路径。这是为了可以在主机上找到容器的配置和根文件系统。

- annotations(map, OPTIONAL) 包含与容器关联的注释列表。如果未提供注释,则此属性可能不存在或为空映射。

在json序列化中必须表现为
```json
{
    "ociVersion"："0.2.0",
    "id"："oci-container1",
    "status"："running",
    "pid"：4422,
    "bundle"："/containers/redis",
    "annotations": {
        "我的钥匙"："我的价值"
    }
}
```

#### 生命周期

1. **调用create**,指定ID和bundle路径
2. 运行时必须根据`config.json`创建容器的运行环境.
    - 如果创建失败必须抛出错误.
    - 当`config.json`中规定的资源正在被创建时,用户定义的行为一定不能被执行.
    - 之后对`config.json`的更新不会影响到容器.
3. 调用`prestart hooks`.(已经弃用,被457代替)
4. 调用`createRuntime hooks`
5. 调用`createContainer hooks`
6. **调用start**, 指定容器ID
7. 调用`startContainer hooks`
8. 运行时运行用户指定的进程
9. 调用`postStart hooks`(warning if failed)
10. 容器进程退出。这可能是由于出错、退出、崩溃或运行时的kill操作被调用而发生.
11. **调用delete**
12. 通过撤销在步骤2中的操作来销毁容器
13. 调用`postStop hooks`(warning if failed)

上述钩子除了9、13外,如果调用失败,运行时必须抛出错误, 停止容器,执行第12步

#### 对于linux系统的特殊配置
**文件描述符**
默认情况下,运行时只会将`stdin, stdout and stderr`的文件描述符设置为开启状态

运行时可以传递额外的描述符去支持socket等功能

一些描述符可能会被重定向到`/dev/null`
**Dev symbolic links**
在生命周期的第二步中,当mount过程结束如果下面的文件存在,就要创建对应的`symlinks`
```
Source	Destination
/proc/self/fd	/dev/fd
/proc/self/fd/0	/dev/stdin
/proc/self/fd/1	/dev/stdout
/proc/self/fd/2	/dev/stderr
```

### 配置文件

#### OCI版本
```json
"ociVersion": "0.1.0"
```

#### ROOT
```json
"root": {
    "path": "rootfs",
    "readonly": true
}
```

#### Mounts
指定了root之外的其他挂载点

- type: linux专有,文件系统类型,比如ext4等
- options: 可选,man page中mount部分规定

```json
"mounts": [
    {
        "destination": "/tmp",
        "type": "tmpfs",
        "source": "tmpfs",
        "options": ["nosuid","strictatime","mode=755","size=65536k"]
    },
    {
        "destination": "/data",
        "type": "none",
        "source": "/volumes/testing",
        "options": ["rbind","rw"]
    }
]
```
#### Process
指定容器运行的进程的参数

**通用**
- terminal (bool, OPTIONAL): 是否为进程分配一个为终端
- consoleSize (object, OPTIONAL): 终端的大小
    - height (uint, REQUIRED)
    - width (uint, REQUIRED)
- cwd (string, REQUIRED): 进程运行所在的根目录
- env (array of strings, OPTIONAL): 环境变量
- args (array of strings, OPTIONAL): 进程运行需要的参数

**POSIX**
- rlimits (array of objects, OPTIONAL) : 可以为进程设置资源限制, 有效值在man page getrlimit(2)
    - type(string, REQUIRED) linux或solaris
    - soft(uint64, REQUIRED) 对相应资源实施的限制值。rlim.rlim_cur必须匹配配置的值。
    - hard(uint64, REQUIRED) 可以由非特权进程设置的软限制的上限。rlim.rlim_max必须匹配配置的值。只有特权进程（例如有CAP_SYS_RESOURCE能力的进程）才能提高硬限制。
- user
    - uid(int, REQUIRED) 指定容器命名空间中的用户 ID 。
    - gid(int, REQUIRED) 指定容器命名空间中的组 ID 。
    - umask(int, OPTIONAL) 指定用户的 umask
    - additionalGids（整数数组,可选）指定要添加到进程的容器命名空间中的其他组 ID。

**Linux**
- apparmorProfile（字符串,可选）不知
- capabilities(object, OPTIONAL) 是一个包含数组的对象,制定了一个进程拥有的能力
    - effective（字符串数组,可选）该effective字段是为流程保留的有效功能数组。
    - bounding（字符串数组,可选）该bounding字段是为进程保留的边界能力数组。
    - inheritable（字符串数组,可选）该inheritable字段是为进程保留的可继承功能数组。
    - permitted（字符串数组,可选）该permitted字段是为进程保留的允许功能的数组。
    - ambient（字符串数组,可选）该ambient字段是为进程保留的环境功能数组。
- noNewPrivileges(bool, OPTIONAL) 设置noNewPrivileges为 true 可以防止进程获得额外的权限。
- oomScoreAdj (int, OPTIONAL) 不知
- selinuxLabel(string, OPTIONAL) 不知

```json
"process": {
    "terminal": true,
    "consoleSize": {
        "height": 25,
        "width": 80
    },
    "user": {
        "uid": 1,
        "gid": 1,
        "umask": 63,
        "additionalGids": [5, 6]
    },
    "env": [
        "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
        "TERM=xterm"
    ],
    "cwd": "/root",
    "args": [
        "sh"
    ],
    "apparmorProfile": "acme_secure_profile",
    "selinuxLabel": "system_u:system_r:svirt_lxc_net_t:s0:c124,c675",
    "noNewPrivileges": true,
    "capabilities": {
        "bounding": [
            "CAP_AUDIT_WRITE",
            "CAP_KILL",
            "CAP_NET_BIND_SERVICE"
        ],
       "permitted": [
            "CAP_AUDIT_WRITE",
            "CAP_KILL",
            "CAP_NET_BIND_SERVICE"
        ],
       "inheritable": [
            "CAP_AUDIT_WRITE",
            "CAP_KILL",
            "CAP_NET_BIND_SERVICE"
        ],
        "effective": [
            "CAP_AUDIT_WRITE",
            "CAP_KILL"
        ],
        "ambient": [
            "CAP_NET_BIND_SERVICE"
        ]
    },
    "rlimits": [
        {
            "type": "RLIMIT_NOFILE",
            "hard": 1024,
            "soft": 1024
        }
    ]
}
```

#### Hostname
容器看到的主机名
```json
"hostname"："mrsdalloway"
```

#### 特定与linux平台的配置
```json
"linux": {
    "namespaces": [
        {
            "type": "pid"
        }
    ]
}
```

#### Hooks
- prestart（已弃用）
- createRuntime: 在创建运行时环境之后但是在pivot_root或任何等效操作之前。
- createContainer: 同上,一般在mount之后,pivot_root之前
- startContainer: 容器启动后,运行用户进程之前
- poststart: 运行用户进程之后,操作返回之前。
- poststop: 容器被删除之后,删除操作返回之前。

```json
"hooks": {
    "prestart": [
        {
            "path": "/usr/bin/fix-mounts",
            "args": ["fix-mounts", "arg1", "arg2"],
            "env":  [ "key1=value1"]
        },
        {
            "path": "/usr/bin/setup-network"
        }
    ],
    "createRuntime": [
        {
            "path": "/usr/bin/fix-mounts",
            "args": ["fix-mounts", "arg1", "arg2"],
            "env":  [ "key1=value1"]
        },
        {
            "path": "/usr/bin/setup-network"
        }
    ],
    "createContainer": [
        {
            "path": "/usr/bin/mount-hook",
            "args": ["-mount", "arg1", "arg2"],
            "env":  [ "key1=value1"]
        }
    ],
    "startContainer": [
        {
            "path": "/usr/bin/refresh-ldcache"
        }
    ],
    "poststart": [
        {
            "path": "/usr/bin/notify-start",
            "timeout": 5
        }
    ],
    "poststop": [
        {
            "path": "/usr/sbin/cleanup.sh",
            "args": ["cleanup.sh", "-f"]
        }
    ]
}
```
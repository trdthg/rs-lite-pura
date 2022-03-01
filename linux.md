# linux容器配置

## 默认文件系统

`Linux ABI`包含系统调用和一些特殊的文件路径，下面的文件路径应该是可用的
|Path    |	Type|
| ----   | --- |
|/proc	 |proc|
|/sys	 |sysfs|
|/dev/pts|	devpts|
|/dev/shm|	tmpfs|

## 命名空间

- type (string, REQUIRED) - 命名空间类型。应该支持以下命名空间类型：
    - pid: 容器内的进程将只能看到同一容器内或同一 pid 命名空间内的其他进程。
    - network: 容器将有自己的网络堆栈。
    - mount: 容器将有一个隔离的安装表。
    - ipc: 容器内的进程将只能通过系统级 IPC 与同一容器内的其他进程通信。
    - uts: 容器将能够拥有自己的主机名和域名。
    - user: 容器将能够将用户和组 ID 从主机重新映射到容器内的本地用户和组。
    - cgroup: 容器将具有 cgroup 层次结构的隔离视图。
- path (string, OPTIONAL) - 命名空间文件路径。此值必须是运行时挂载命名空间中的绝对路径。

    如果path未指定，运行时必须创建一个类型为 的新容器命名空间type。

```json
"namespaces": [
    {
        "type": "pid",
        "path": "/proc/1234/ns/pid"
    },
    {
        "type": "network",
        "path": "/var/run/netns/neta"
    },
    {
        "type": "mount"
    },
    {
        "type": "ipc"
    },
    {
        "type": "uts"
    },
    {
        "type": "user"
    },
    {
        "type": "cgroup"
    }
]
```

## 用户命名空间映射
- uidMappings（对象数组，可选）描述从主机到容器的用户命名空间 uid 映射。

    每个条目具有以下结构：

    - containerID (uint32, REQUIRED) - 是容器中的起始 uid/gid。
    - hostID (uint32, REQUIRED) - 是主机上要映射到containerID的起始 uid/gid 。
    - size (uint32, REQUIRED) - 是要映射的 id 数。

运行时不应该修改引用文件系统的所有权来实现映射。请注意，映射条目的数量可能受内核限制。

```json
"uidMappings": [
    {
        "containerID": 0,
        "hostID": 1000,
        "size": 32000
    }
],
"gidMappings": [
    {
        "containerID": 0,
        "hostID": 1000,
        "size": 32000
    }
]
```

## 设备


- devices（对象数组，可选）列出容器中必须可用的设备。运行时可以根据自己的喜好提供它们（mknod通过从运行时挂载命名空间绑定挂载，使用符号链接等）。
    - type (string, REQUIRED) - 设备类型：c, b,u或p. 参考mknod(1)
    - path (string, REQUIRED) - 容器内设备的完整路径。如果已经存在与请求的设备不匹配的文件，则运行时必须生成错误。
    - major, minor (int64, REQUIRED unless type is p) - 设备的主要、次要编号。
    - fileMode (uint32, OPTIONAL) - 设备的文件模式。您还可以使用`cgroups`控制对设备的访问。
    - uid （uint32，可选） -容器命名空间中设备所有者的 ID 。
    - gid (uint32, OPTIONAL) -容器命名空间中设备组的 id 。

相同type，major并且minor不应该用于多个设备。
```json
"devices": [
    {
        "path": "/dev/fuse",
        "type": "c",
        "major": 10,
        "minor": 229,
        "fileMode": 438,
        "uid": 0,
        "gid": 0
    },
    {
        "path": "/dev/sda",
        "type": "b",
        "major": 8,
        "minor": 0,
        "fileMode": 432,
        "uid": 0,
        "gid": 0
    }
]
```

### 默认设备
```
/dev/null
/dev/zero
/dev/full
/dev/random
/dev/urandom
/dev/tty
/dev/console is set up if terminal is enabled in the config by bind mounting the pseudoterminal pty to /dev/console.
/dev/ptmx. A bind-mount or symlink of the container's /dev/pts/ptmx.
```
### 控制组(Control groups)
- cgroupsPath (string, OPTIONAL) cgroups 的路径
- resources
    - devices: 可用设备列表
        - allow (boolean, REQUIRED) - 输入是允许还是拒绝。
        - type (string, OPTIONAL) - 设备类型：a(all), c(char), or b(block)。未设置的值表示“全部”，映射到a.
        - major, minor (int64, OPTIONAL) -设备的主要、次要编号。未设置的值表示“全部”，映射到*文件系统 API中。
        - access （字符串，可选） - 设备的 cgroup 权限。r(read)、w(write) 和m(mknod)的组合。
    - memory: 内存
        - limit (int64, OPTIONAL) - 设置内存使用限制
        - reservation (int64, OPTIONAL) - 设置内存使用的软限制
        - swap (int64, OPTIONAL) - 设置内存+交换使用限制
        - kernel (int64, OPTIONAL, NOT RECOMMENDED) - 设置内核内存的硬限制
        - kernelTCP (int64, OPTIONAL, NOT RECOMMENDED) - 设置内核 TCP 缓冲内存的硬限制

        以下属性不指定内存限制，但由memory控制器覆盖：

        - swappiness (uint64, OPTIONAL) - 设置 vmscan 的 swappiness 参数（参见 sysctl 的 vm.swappiness） 值是从 0 到 100。更高意味着更多的 swappy。
        - disableOOMKiller (bool, OPTIONAL) - 超过内存限制的进程会被杀死
        - useHierarchy (bool, OPTIONAL) - 未知
    - cpu: CPU
        - shares (uint64, OPTIONAL) - 指定 cgroup 中任务可用的 CPU 时间的相对份额
        - quota (int64, OPTIONAL) - 指定一个 cgroup 中的所有任务在一个时间段内可以运行的总时间（以微秒为单位）（定义period如下）
        - period (uint64, OPTIONAL) - 以微秒为单位指定一个 cgroup 对 CPU 资源的访问应该重新分配的频率（仅限 CFS 调度程序）
        - realtimeRuntime (int64, OPTIONAL) - 指定 cgroup 中的任务可以访问 CPU 资源的最长连续时间段（以微秒为单位）
        - realtimePeriod (uint64, OPTIONAL) - 与实时调度程序相同period但仅适用于实时调度程序
        - cpus （字符串，可选） - 容器将在其中运行的 CPU 列表
        - mems （字符串，可选） - 容器将在其中运行的内存节点列表


```json
{
"cgroupsPath": "/myRuntime/myContainer",
"resources": {
    "memory": {
        "limit": 536870912,
        "reservation": 536870912,
        "swap": 536870912,
        "kernel": -1,
        "kernelTCP": -1,
        "swappiness": 0,
        "disableOOMKiller": false
    },
    "devices": [
        "devices": [
            {
                "allow": false,
                "access": "rwm"
            },
            {
                "allow": true,
                "type": "c",
                "major": 10,
                "minor": 229,
                "access": "rw"
            },
            {
                "allow": true,
                "type": "b",
                "major": 8,
                "minor": 0,
                "access": "r"
            }
        ]
    ]
}
}
```

## 其他

- Block IO
    不同设备的IO权重
- Network
- Huge page limits
- PIDs
    - pids设置任务最大数量
        ```json
        "pids": {
            "limit": 32771
        }
        ```
- RDMA
- Unified
- IntelRdt
- Sysctl: 允许在容器运行时修改内核参数
- Seccomp
- The Container Process State
- Rootfs Mount Propagation
- Masked Paths
- Readonly Paths
- Mount Label
- Personality
```json

    "linux": {
        "devices": [
            {
                "path": "/dev/fuse",
                "type": "c",
                "major": 10,
                "minor": 229,
                "fileMode": 438,
                "uid": 0,
                "gid": 0
            },
            {
                "path": "/dev/sda",
                "type": "b",
                "major": 8,
                "minor": 0,
                "fileMode": 432,
                "uid": 0,
                "gid": 0
            }
        ],
        "uidMappings": [
            {
                "containerID": 0,
                "hostID": 1000,
                "size": 32000
            }
        ],
        "gidMappings": [
            {
                "containerID": 0,
                "hostID": 1000,
                "size": 32000
            }
        ],
        "sysctl": {
            "net.ipv4.ip_forward": "1",
            "net.core.somaxconn": "256"
        },
        "cgroupsPath": "/myRuntime/myContainer",
        "resources": {
            "network": {
                "classID": 1048577,
                "priorities": [
                    {
                        "name": "eth0",
                        "priority": 500
                    },
                    {
                        "name": "eth1",
                        "priority": 1000
                    }
                ]
            },
            "pids": {
                "limit": 32771
            },
            "hugepageLimits": [
                {
                    "pageSize": "2MB",
                    "limit": 9223372036854772000
                },
                {
                    "pageSize": "64KB",
                    "limit": 1000000
                }
            ],
            "memory": {
                "limit": 536870912,
                "reservation": 536870912,
                "swap": 536870912,
                "kernel": -1,
                "kernelTCP": -1,
                "swappiness": 0,
                "disableOOMKiller": false
            },
            "cpu": {
                "shares": 1024,
                "quota": 1000000,
                "period": 500000,
                "realtimeRuntime": 950000,
                "realtimePeriod": 1000000,
                "cpus": "2-3",
                "mems": "0-7"
            },
            "devices": [
                {
                    "allow": false,
                    "access": "rwm"
                },
                {
                    "allow": true,
                    "type": "c",
                    "major": 10,
                    "minor": 229,
                    "access": "rw"
                },
                {
                    "allow": true,
                    "type": "b",
                    "major": 8,
                    "minor": 0,
                    "access": "r"
                }
            ],
            "blockIO": {
                "weight": 10,
                "leafWeight": 10,
                "weightDevice": [
                    {
                        "major": 8,
                        "minor": 0,
                        "weight": 500,
                        "leafWeight": 300
                    },
                    {
                        "major": 8,
                        "minor": 16,
                        "weight": 500
                    }
                ],
                "throttleReadBpsDevice": [
                    {
                        "major": 8,
                        "minor": 0,
                        "rate": 600
                    }
                ],
                "throttleWriteIOPSDevice": [
                    {
                        "major": 8,
                        "minor": 16,
                        "rate": 300
                    }
                ]
            }
        },
        "rootfsPropagation": "slave",
        "seccomp": {
            "defaultAction": "SCMP_ACT_ALLOW",
            "architectures": [
                "SCMP_ARCH_X86",
                "SCMP_ARCH_X32"
            ],
            "syscalls": [
                {
                    "names": [
                        "getcwd",
                        "chmod"
                    ],
                    "action": "SCMP_ACT_ERRNO"
                }
            ]
        },
        "namespaces": [
            {
                "type": "pid"
            },
            {
                "type": "network"
            },
            {
                "type": "ipc"
            },
            {
                "type": "uts"
            },
            {
                "type": "mount"
            },
            {
                "type": "user"
            },
            {
                "type": "cgroup"
            }
        ],
        "maskedPaths": [
            "/proc/kcore",
            "/proc/latency_stats",
            "/proc/timer_stats",
            "/proc/sched_debug"
        ],
        "readonlyPaths": [
            "/proc/asound",
            "/proc/bus",
            "/proc/fs",
            "/proc/irq",
            "/proc/sys",
            "/proc/sysrq-trigger"
        ],
        "mountLabel": "system_u:object_r:svirt_sandbox_file_t:s0:c715,c811"
    },
```
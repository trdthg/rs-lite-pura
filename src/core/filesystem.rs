use std::{
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    str::FromStr, fs,
};

use nix::{
    mount::{mount, umount, MsFlags},
    sys::stat::{makedev, mknod, Mode, SFlag},
    unistd::{chdir, chown, pivot_root, Gid, Uid},
};

use crate::{
    error::{Error, Result},
    oci::{Device, Mount},
};

pub fn symlinks_defaults(rootfs: &Path) -> Result<()> {
    let default_symlinks = [
        ("/proc/self/fd", "dev/fd"),
        ("/proc/self/fd/0", "dev/stdin"),
        ("/proc/self/fd/1", "dev/stdout"),
        ("/proc/self/fd/2", "dev/stderr"),
    ];

    for (src, dest) in default_symlinks {
        symlink(src, rootfs.join(dest))?;
    }
    Ok(())
}

pub fn default_devices() -> Vec<Device> {
    vec![
        Device {
            path: String::from("/dev/null"),
            device_type: String::from("c"),
            major: 1,
            minor: 3,
            file_mode: Some(0o066),
            uid: Some(0),
            gid: Some(0),
        },
        Device {
            path: String::from("/dev/zero"),
            device_type: String::from("c"),
            major: 1,
            minor: 5,
            file_mode: Some(0o066),
            uid: Some(0),
            gid: Some(0),
        },
        Device {
            path: String::from("/dev/full"),
            device_type: String::from("c"),
            major: 1,
            minor: 7,
            file_mode: Some(0o066),
            uid: Some(0),
            gid: Some(0),
        },
        Device {
            path: String::from("/dev/random"),
            device_type: String::from("c"),
            major: 1,
            minor: 8,
            file_mode: Some(0o066),
            uid: Some(0),
            gid: Some(0),
        },
        Device {
            path: String::from("/dev/urandom"),
            device_type: String::from("c"),
            major: 1,
            minor: 9,
            file_mode: Some(0o066),
            uid: Some(0),
            gid: Some(0),
        },
        Device {
            path: String::from("/dev/tty"),
            device_type: String::from("c"),
            major: 5,
            minor: 0,
            file_mode: Some(0o066),
            uid: Some(0),
            gid: Some(0),
        },
        Device {
            path: String::from("/dev/ptmx"),
            device_type: String::from("c"),
            major: 5,
            minor: 2,
            file_mode: Some(0o066),
            uid: Some(0),
            gid: Some(0),
        },
    ]
}

pub fn to_sflag(flag: &str) -> Result<SFlag> {
    let sflag = match flag {
        "c" | "u" => SFlag::S_IFCHR,
        "b" => SFlag::S_IFBLK,
        "p" => SFlag::S_IFIFO,
        _ => {
            return Err(Error::StringError(
                "translate {flag} to sflag failed!".into(),
            ))
        }
    };
    Ok(sflag)
}

pub fn create_dev(dev: &Device, rootfs: &Path) -> Result<()> {
    let path = rootfs.join(&dev.path.trim_start_matches("/"));

    // 该函数用来创建文件，[文件类型 文件权限 设备号]
    mknod(
        &path,
        to_sflag(&dev.device_type)?,
        Mode::from_bits_truncate(dev.file_mode.unwrap_or(0o066).try_into()?),
        makedev(dev.major, dev.minor),
    )?;

    // 修改设备的拥有者
    if let Some(uid) = dev.uid {
        chown(path.as_os_str(), Some(Uid::from_raw(uid)), None)?;
    }
    // 修改设备的拥有组
    if let Some(gid) = dev.gid {
        chown(path.as_os_str(), None, Some(Gid::from_raw(gid)))?;
    }
    Ok(())
}

pub fn bind_dev(dev: &Device) -> Result<()> {
    let path = PathBuf::from_str(&dev.path)?;
    mount(
        Some(&path),
        &path,
        None::<&str>,
        MsFlags::MS_BIND,
        None::<&str>,
    );
    Ok(())
}

pub fn create_default_devices(rootfs: &Path) -> Result<()> {
    let devices = default_devices();
    for dev in devices {
        create_dev(&dev, rootfs)?;
    }
    Ok(())
}

pub fn create_devices(devices: &Vec<Device>, rootfs: &Path) -> Result<()> {
    for dev in devices {
        create_dev(&dev, rootfs)?;
    }
    Ok(())
}

/// mount --bind alpine alpine
/// unshare -m
/// cd alpine
/// mkdir oldroot
/// pivot_root . oldroot/
/// cd /
/// unmount -l oldroot/
/// rm -rf oldroot/
///
/// 注意，mount_rootfs和pivot_rootfs都是在新创建的挂载命名空间中调用的。
pub fn mount_rootfs(rootfs: &Path) -> Result<()> {
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE | MsFlags::MS_REC,
        None::<&str>,
    )?;
    mount::<Path, Path, str, str>(
        Some(&rootfs),
        &rootfs,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )?;
    Ok(())
}

pub fn pivot_rootfs(rootfs: &Path) -> Result<()> {
    chdir(rootfs)?;
    let oldroot = rootfs.join("oldroot");
    std::fs::create_dir_all(oldroot.as_os_str())?;
    pivot_root(rootfs.as_os_str(), oldroot.as_os_str())?;
    umount(oldroot.as_os_str())?;
    std::fs::remove_dir_all(oldroot.as_os_str())?;
    chdir("/")?;
    Ok(())
}

pub fn mount_devices(mounts: &Vec<Mount>, rootfs: &Path) -> Result<()> {
    for m in mounts {
        let mut flags = MsFlags::empty();

        let dest = rootfs.join(m.destination.trim_start_matches("/"));
        if !Path::new(&dest).exists() {
            fs::create_dir_all(dest)?;
        }

        if m.mount_type.as_ref().ok_or(Error::StringError("")) == "bind" {

        }
    }
    Ok(())
}

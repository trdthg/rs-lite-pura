use std::{os::unix::fs::symlink, path::Path};

use nix::{
    mount::{mount, umount, MsFlags},
    unistd::{chdir, pivot_root},
};

use crate::error::Result;

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
    Ok(())
}

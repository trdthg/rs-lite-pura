use nix::{fcntl::OFlag, pty::posix_openpt};

use crate::{Error, Result};

pub struct Pty {
    master: nix::pty::PtyMaster,
    slave_name: String,
}

impl Pty {
    pub fn new() -> Result<Self> {
        let master = posix_openpt(OFlag::O_RDWR).map_err(|e| Error::StringError(format!("")))?;
        
    }
}

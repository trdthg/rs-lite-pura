use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {}", source)]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("convert error: {}", source)]
    Convert {
        #[from]
        source: std::convert::Infallible,
    },

    #[error("nix error: {}", source)]
    Nix {
        #[from]
        source: nix::Error,
    },

    #[error("error msg: `{0}`")]
    StringError(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },

    #[error("unknown data store error")]
    Unknown,
}

pub type Result<T, R = Error> = std::result::Result<T, R>;

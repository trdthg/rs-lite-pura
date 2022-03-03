mod core;
mod error;
mod logger;
mod oci;

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use oci::Create;

use crate::logger::ContainerLogger;

#[derive(Parser, Debug)]
#[clap(author = "trdthg", version = "v0.0.0", about = "a mini container runtime", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(long, help = "runtime root for the container state")]
    root: Option<String>,
    #[clap(long, help = "location of the log file")]
    log: Option<String>,
    #[clap(long = "log-format", help = "log format (e.q. json, txt)")]
    log_format: Option<String>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "create a new continaer")]
    Create {
        #[clap(long, help = "bundle path of the container")]
        bundle: String,
        #[clap(long, help = "ID of the container")]
        id: String,
        #[clap(
            long = "console-socket",
            help = "UNIX socket to send the pty master fd, if terminal: true"
        )]
        console_socket: Option<String>,
        #[clap(
            long = "pid-file",
            help = "file path to write the container process PID"
        )]
        pid_file: Option<String>,
    },
    #[clap(about = "start a continaer")]
    Start {
        #[clap(long, help = "start the container")]
        id: String,
    },
    #[clap(about = "kill a continaer")]
    Kill {
        #[clap(long, help = "container id")]
        id: String,
        #[clap(long, help = "optional signal")]
        signal: Option<String>,
    },
    #[clap(about = "delete a continaer")]
    Delete {
        #[clap(long, help = "container id")]
        id: String,
    },
    #[clap(about = "watch the state of a container")]
    State {
        #[clap(long, help = "container id")]
        id: String,
    },
}

fn main() -> crate::error::Result<()> {
    let cli = Cli::parse();
    let log_path: String = match cli.log {
        Some(ref log) => log.to_string(),
        None => String::from("/tmp/pura/unknown.log"),
    };
    ContainerLogger::init(&log_path, log::Level::Info)?;
    let root = match cli.root {
        Some(ref r) => r.to_owned(),
        None => "/tmp/pura".to_owned(),
    };
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    println!("{cli:?}");
    match cli.command {
        Commands::Create {
            id,
            bundle,
            console_socket,
            pid_file,
        } => create(Create {
            id: id.to_string(),
            bundle: bundle.to_string(),
            console_socket: console_socket.to_owned(),
            pid_file: pid_file.to_owned(),
            root,
        }),
        Commands::Start { id } => todo!(),
        Commands::Kill { id, signal } => todo!(),
        Commands::Delete { id } => todo!(),
        Commands::State { id } => todo!(),
    }
    Ok(())
}

pub fn create(create: Create) {
    todo!()
}

// #[clap(
//     short,
//     long,
//     value_name = PORT_FORMAT,
//     default_value = DEFAULT_LISTENING_ADDRESS,
//     help = "Sets the listening address",
// )]

// #[clap(
//     arg_enum,
//     long,
//     help = "Sets the storage engine",
//     value_name = "ENGINE_NAME"
// )]
// engine: Option<Engine>,
// }

// #[allow(non_camel_case_types)]
// #[derive(ArgEnum, Debug, Clone, Copy, PartialEq, Eq)]
// enum Engine {
// kvs,
// sled,
// }

// impl std::fmt::Display for Engine {
// fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     match *self {
//         Engine::kvs => write!(f, "kvs"),
//         Engine::sled => write!(f, "sled"),
//     }
// }
// }

// impl FromStr for Engine {
// type Err = KvsError;

// fn from_str(s: &str) -> Result<Self> {
//     match s {
//         "kvs" => Ok(Engine::kvs),
//         "sled" => Ok(Engine::sled),
//         _ => Err(KvsError::NoSuchEngine),
//     }
// }
// }

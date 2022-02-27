use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author = "trdthg", version = "v0.0.0", about = "a mini container runtime", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "create a new continaer")]
    Create {
        #[clap(long, help = "Sets the container bundle")]
        bundle: String,
        #[clap(long, help = "Sets the container id")]
        id: String,
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

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Create { .. } => {}
        Commands::Start { id } => todo!(),
        Commands::Kill { id, signal } => todo!(),
        Commands::Delete { id } => todo!(),
        Commands::State { id } => todo!(),
        _ => unreachable!(),
    }
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

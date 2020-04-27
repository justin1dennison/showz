use structopt::StructOpt;

pub(crate) const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const SCHEMA_GLOB: &str = "schema/*sql";

mod handlers;
mod types;

use handlers::{config, episode, init, testing};
use types::{CliResult, EpisodeCommand};

#[derive(StructOpt, Debug)]
enum Opt {
    Init {
        #[structopt(default_value = ".")]
        dir: String,
    },
    Episode {
        #[structopt(subcommand)]
        command: EpisodeCommand,
    },
    Config,
    Testing,
}

#[async_std::main]
async fn main() -> CliResult {
    match Opt::from_args() {
        Opt::Init { dir } => init(dir).await,
        Opt::Episode { command } => episode(command).await,
        Opt::Config => config().await,
        Opt::Testing => testing().await,
    }
}
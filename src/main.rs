use structopt::StructOpt;

pub(crate) const APP_NAME: &str = env!("CARGO_PKG_NAME");

mod handlers;
mod types;

use handlers::{config, episode, init, resource, testing, people};
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
    Resource {},
    Config,
    Person,
    Testing,
}

#[async_std::main]
async fn main() -> CliResult {
    env_logger::init();
    match Opt::from_args() {
        Opt::Init { dir } => init(dir).await,
        Opt::Episode { command } => episode(command).await,
        Opt::Config => config().await,
        Opt::Resource {} => resource().await,
        Opt::Person => people().await,
        Opt::Testing => testing().await,
    }
}

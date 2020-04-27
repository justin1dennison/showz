use anyhow::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum EpisodeCommand {
    Create {
        title: String,
        topic: String,
        objectives: Vec<String>,
    },
    Delete,
    Update,
}

pub type CliResult = Result<(), Error>;

use crate::types::CliResult;
use crate::EpisodeCommand;
use crate::{APP_NAME, SCHEMA_GLOB};
use askama::Template;
use async_std::fs;
use async_std::path::PathBuf;
use async_std::prelude::*;
use glob::glob;
use rusqlite::{params, Connection};

#[derive(Template)]
#[template(path = "config/default.toml.j2")]
struct Config;

pub async fn init(dir: String) -> CliResult {
    let path: &PathBuf = &[dir, format!(".{}", APP_NAME)].iter().collect::<PathBuf>();
    if !path.is_dir().await {
        fs::create_dir_all(path).await?;
    };
    let db_path = path.join("db");
    let connection = Connection::open(db_path)?;
    let schemas = glob(SCHEMA_GLOB)?;
    for schema in schemas {
        let migration = fs::read_to_string(schema?).await?;
        connection.execute(&migration, params![])?;
    }
    let config = Config.render()?;
    let config_path = path.join("config.toml");
    let mut config_file = fs::File::create(config_path).await?;
    config_file.write_all(&config.as_bytes()).await?;
    Ok(())
}

pub async fn testing() -> CliResult {
    Ok(())
}

pub async fn episode(_command: EpisodeCommand) -> CliResult {
    println!("Episodes");
    Ok(())
}

pub async fn config() -> CliResult {
    println!("Config");
    Ok(())
}
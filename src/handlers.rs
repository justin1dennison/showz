use crate::types::CliResult;
use crate::EpisodeCommand;
use crate::APP_NAME;
use askama::Template;
use async_std::fs;
use async_std::path::PathBuf;
use async_std::prelude::*;
use glob::glob;
use log::{error, info, warn};
use rusqlite::{params, Connection};

#[derive(Template)]
#[template(path = "config/default.toml.j2")]
struct Config;

async fn is_initialized(dir: String) -> bool {
    let path: &PathBuf = &[dir, format!(".{}", APP_NAME)].iter().collect::<PathBuf>();
    return path.exists().await && path.is_dir().await;
}

pub async fn init(dir: String) -> CliResult {
    let path: &PathBuf = &[dir, format!(".{}", APP_NAME)].iter().collect::<PathBuf>();
    if !path.is_dir().await {
        fs::create_dir_all(path).await?;
    };
    let db_path = path.join("db");
    let connection = Connection::open(db_path)?;
    let schema_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("schemas")
        .join("*.sql");
    let schemas = glob(schema_dir.to_str().unwrap())?;
    for schema in schemas {
        let schema = schema?;
        let migration = fs::read_to_string(&schema).await?;
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
    if !is_initialized(".".to_string()).await {
        return Err(anyhow::anyhow!("Project Not Initialized"));
    }
    println!("episodes!");
    Ok(())
}

pub async fn config() -> CliResult {
    if !is_initialized(".".to_string()).await {
        return Err(anyhow::anyhow!("Project Not Initialized"));
    }
    Ok(())
}

pub async fn resource() -> CliResult {
    if !is_initialized(".".to_string()).await {
        return Err(anyhow::anyhow!("Project Not Initialized"));
    }
    Ok(())
}

pub async fn people() -> CliResult {
    if !is_initialized(".".to_string()).await {
        return Err(anyhow::anyhow!("Project Not Initialized"));
    }
    Ok(())
}

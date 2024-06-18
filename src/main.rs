#![deny(clippy::all, unsafe_code)]

use crate::config::Config;
use anyhow::Result;
use log::{debug, error};
use rfesi::prelude::{Esi, EsiBuilder};
use std::{env, process, time::SystemTime};

mod config;
mod eve_data;
mod interface;
mod state;

fn setup_logging() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("app.log")?)
        .apply()?;
    Ok(())
}

async fn setup_esi(config: &Config) -> Result<Esi> {
    let esi = EsiBuilder::new()
        .user_agent("github.com/celeo/evemapping")
        .client_id(&config.sso_client_id)
        .client_secret(&config.sso_client_secret)
        .callback_url(&config.sso_callback_url)
        .build()?;
    Ok(esi)
}

#[tokio::main]
async fn main() {
    if let Err(e) = setup_logging() {
        error!("Could not set up logging: {e}");
        process::exit(1);
    }

    debug!("Loading config");
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            error!("Could not load config: {e}");
            process::exit(1);
        }
    };
    debug!("Setting up ESI");
    let esi = match setup_esi(&config).await {
        Ok(e) => e,
        Err(e) => {
            error!("Could not set up connection to ESI: {e}");
            process::exit(1);
        }
    };

    debug!("Starting");
    if let Err(e) = interface::run(esi).await {
        error!("An error occurred during running: {e}");
        process::exit(1);
    }
}

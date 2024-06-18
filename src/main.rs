#![deny(clippy::all, unsafe_code)]

use anyhow::Result;
use log::info;
use std::{env, time::SystemTime};

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

#[tokio::main]
async fn main() {
    setup_logging().expect("Could not set up logging");
    info!("Starting");
    interface::run().await.expect("Could not set up interface");
}

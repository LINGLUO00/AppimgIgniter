#[allow(unused_imports)]
use crate::modules::logging::logging;
use anyhow::{Context, Result};
use tracing::{debug, error, info, trace, warn};

pub async fn test_logging() -> Result<()> {
    logging::init_dev().context("fail:init loggging fail")?;
    println!("init logging success");
    test().await?;
    Ok(())
}

async fn test() -> Result<()> {
    info!("start test logging module");

    //test different kinds of log levels
    trace!("this is a trace level log");
    debug!("this is a debug level log");
    warn!("this is a warn level log");
    error!("this is a error level log");

    //test structured logging
    let app_name = "AppimgIgniter";
    let version = "0.1.0";
    info!(app_name = app_name, version = version, "app startup",);
    Ok(())
}

use anyhow::{Context, Result};
use clap::{Command, Parser};
use std::env;
use std::path::PathBuf;
use tokio::signal;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{EnvFilter, fmt};
pub mod modules;
pub mod test;
use test::logging_test::logging_test;
#[allow(unused_imports)]
#[tokio::main]
async fn main() -> Result<()> {
    logging_test::test_logging().await?;
    Ok(())
}

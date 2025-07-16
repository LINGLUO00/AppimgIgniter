use crate::modules::logging::logging;
use crate::test::logging_test;
use anyhow::{Context, Result};
use tracing::info;

pub mod modules;
pub mod test;
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    logging::init_dev().context("初始化日志失败")?;
    info!("程序退出");
    Ok(())
}

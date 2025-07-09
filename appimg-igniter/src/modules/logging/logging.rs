use anyhow::Result;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
#[allow(unused_imports)]
//默认初始化
pub fn init() -> Result<()> {
    let filter = EnvFilter::from_default_env().add_directive("appimg_igniter=info".parse()?);
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(filter)
        .init();

    Ok(())
}

//为开发调试启动详细的日志配置
pub fn init_dev() -> Result<()> {
    let filter = EnvFilter::from_default_env().add_directive("appimg_igniter=info".parse()?);
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .pretty(),
        )
        .with(filter)
        .init();

    Ok(())
}

//为产品环境启动粗略日志配置
pub fn init_prod() -> Result<()> {
    let filter = EnvFilter::from_default_env().add_directive("appimg_igniter=info".parse()?);
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .pretty()
                .compact(),
        )
        .with(filter)
        .init();

    Ok(())
}

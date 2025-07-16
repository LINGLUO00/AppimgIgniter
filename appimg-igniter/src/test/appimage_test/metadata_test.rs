use crate::modules::appimage::metadata::extract_metadata;
use anyhow::Result;
use once_cell::sync::Lazy;
use std::path::Path;
use tracing::info;

static LOG_INIT: Lazy<()> = Lazy::new(|| {
    let _ = crate::modules::logging::logging::init_dev();
});
#[tokio::test]
async fn test_extract_metadata() -> Result<()> {
    Lazy::force(&LOG_INIT);
    let image_path = Path::new("/home/hyl/Documents/Test/Cursor-1.2.1-x86_64.AppImage");
    info!("开始解析 AppImage: {}", image_path.display());

    let metadata = extract_metadata(image_path)?;
    info!("解析结果: {:#?}", metadata);

    // 基础断言：Name 字段应非空，且 desktop_entry 行数 > 0
    assert!(
        !metadata.name.is_empty(),
        "Name 字段应当存在于 .desktop 文件中"
    );
    assert!(
        !metadata.desktop_entry.is_empty(),
        ".desktop 文件内容不应为空"
    );

    Ok(())
}

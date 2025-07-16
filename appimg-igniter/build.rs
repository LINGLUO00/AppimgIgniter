use std::env;
use std::path::PathBuf;

fn main() {
    // 1. 通过 pkg-config 查找系统里的 libappimage
    pkg_config::Config::new()
        .probe("libappimage")
        .expect("找不到 libappimage 库");

    // 2. 通知 Cargo 链接该库
    println!("cargo:rustc-link-lib=appimage");

    // 3. 生成 Rust 绑定
    let bindings = bindgen::Builder::default()
        .header("src/clib/wrapper.h") // 注意相对路径
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("无法生成 bindings");

    // 4. 写入 $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("无法写入 bindings.rs");
}

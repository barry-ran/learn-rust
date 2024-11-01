use std::path::PathBuf;
use std::env;

// 通过build.rs指定c库的search路径 https://stackoverflow.com/a/26254062
fn main() {
    // 参数说明 https://doc.rust-lang.org/cargo/reference/build-scripts.html    

    #[cfg(target_os = "macos")]
    let yuv_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/libyuv/mac");    

    #[cfg(target_os = "windows")]
    let yuv_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/libyuv/win");
    
    println!(r"cargo:rustc-link-search={}", yuv_path.to_string_lossy());

    // test
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_dir = env::var("CARGO").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("test out_dir: {}", out_dir);
    println!("test cargo_dir: {}", cargo_dir);
    println!("test manifest_dir: {}", manifest_dir);           
}
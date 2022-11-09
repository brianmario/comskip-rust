fn main() {
    println!("cargo:rustc-link-search=/usr/local/opt/ffmpeg@4/lib");
    println!("cargo:rustc-link-search=.");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=dylib=argtable2");
    println!("cargo:rustc-link-lib=static=avutil");
    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=static=swscale");
}

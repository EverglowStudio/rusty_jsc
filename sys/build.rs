#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}

#[cfg(target_os = "linux")]
fn main() {
    pkg_config::probe_library("javascriptcoregtk-4.1").unwrap();
}

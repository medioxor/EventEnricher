use std::process::Command;
use std::path::PathBuf;
use std::env;
use bindgen;

fn main() -> Result<(), wdk_build::ConfigError> {
    let manifest_file = "EventEnrich.man";
    let mc_exe = "mc.exe";
    let out_path = PathBuf::from(
        env::var_os("OUT_DIR")
            .expect("the environment variable OUT_DIR is undefined")
    );

    let status = Command::new(mc_exe)
        .arg("-km")
        .arg("EventEnrich.man")
        .status()
        .expect("Failed to execute mc.exe");

    if !status.success() {
        panic!("mc.exe failed with status: {:?}", status);
    }
    let file_path = "EventEnrich.h";
    let content = std::fs::read_to_string(file_path)
        .expect("Failed to read EventEnrich.h");
    let modified_content = content.replace("__declspec(selectany)", "");
    std::fs::write(file_path, modified_content)
        .expect("Failed to write EventEnrich.h");

    println!("cargo:rerun-if-changed={}", manifest_file);
    bindgen::Builder::default()
        .header("src/etw_provider.h")
        .ctypes_prefix("cty")
        .use_core()
        .clang_macro_fallback()
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("etw.rs"))
        .unwrap();

    cc::Build::new()
        .flag("/kernel")
        .file("src/etw_provider.c")
        .compile("etw_provider");

    wdk_build::configure_wdk_binary_build()
}
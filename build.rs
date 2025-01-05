use std::process::Command;
use std::path::PathBuf;
use std::env;
use bindgen;
use std::error::Error;
use winreg::RegKey;
use winreg::enums::HKEY_LOCAL_MACHINE;

pub enum DirectoryType {
    Include,
    Library,
}

pub fn get_windows_kits_dir() -> Result<PathBuf, Box<dyn Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = r"SOFTWARE\Microsoft\Windows Kits\Installed Roots";
    let dir: String = hklm.open_subkey(key)?.get_value("KitsRoot10")?;

    Ok(dir.into())
}

pub fn get_km_dir(dir_type: DirectoryType) -> Result<PathBuf, Box<dyn Error>> {
    // We first append lib to the path and read the directory..
    let dir = get_windows_kits_dir()?
        .join(match dir_type {
            DirectoryType::Include => "Include",
            DirectoryType::Library => "Lib",
        })
        .read_dir()?;

    // In the lib directory we may have one or more directories named after the version of Windows,
    // we will be looking for the highest version number.
    let dir = dir
        .filter_map(|dir| dir.ok())
        .map(|dir| dir.path())
        .filter(|dir| {
            dir.components()
                .last()
                .and_then(|c| c.as_os_str().to_str())
                .map(|c| c.starts_with("10.") && dir.join("km").is_dir())
                .unwrap_or(false)
        })
        .max().unwrap();

    // Finally append km to the path to get the path to the kernel mode libraries.
    Ok(dir.join("km"))
}

fn main() -> Result<(), wdk_build::ConfigError> {
    let manifest_file = "EventEnricher.man";
    let out_path = PathBuf::from(
        env::var_os("OUT_DIR")
            .expect("the environment variable OUT_DIR is undefined")
    );

    let include_dir = get_km_dir(DirectoryType::Include).unwrap();

    let mut status = Command::new("mc.exe")
        .arg("-km")
        .arg("EventEnricher.man")
        .status()
        .expect("Failed to execute mc.exe");

    if !status.success() {
        panic!("mc.exe failed with status: {:?}", status);
    }

    status = Command::new("rc.exe")
        .arg("EventEnricher.rc")
        .status()
        .expect("Failed to execute rc.exe");

    if !status.success() {
        panic!("rc.exe failed with status: {:?}", status);
    }

    status = Command::new("link.exe")
        .arg("/dll")
        .arg("/noentry")
        .arg("/machine:x64")
        .arg("EventEnricher.res")
        .arg("/out:EventEnricherProvider.dll")
        .status()
        .expect("Failed to execute link.exe");

    if !status.success() {
        panic!("link.exe failed with status: {:?}", status);
    }

    println!("cargo:rerun-if-changed={}", manifest_file);
    bindgen::Builder::default()
        .header("src/etw_provider.h")
        .ctypes_prefix("cty")
        .use_core()
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("etw.rs"))
        .unwrap();
    
    cc::Build::new()
        .flag("/kernel")
        .file("src/etw_provider.c")
        .include(include_dir)
        .compile("etw_provider");

    wdk_build::configure_wdk_binary_build()
}
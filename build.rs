use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const TAPES_CRATE: &str = "tapes-dotfiles";
const TAPES_VERSION: &str = "0.1.9-beta";

fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    bundle_tapes();
}

fn bundle_tapes() {
    println!("cargo:rerun-if-env-changed=TAPES_BINARY");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let bundled = out.join("tapes");

    if let Some(prebuilt) = env::var_os("TAPES_BINARY") {
        println!("cargo:rerun-if-changed={}", prebuilt.to_string_lossy());
        fs::copy(prebuilt, bundled).expect("couldn't copy TAPES_BINARY");
        return;
    }

    let root = out.join(format!("{TAPES_CRATE}-{TAPES_VERSION}"));
    let built = root.join("bin/tapes");
    if !built.is_file() {
        let status = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
            .args(["install", TAPES_CRATE, "--locked", "--version", &format!("={TAPES_VERSION}")])
            .arg("--root")
            .arg(&root)
            .env("CARGO_TARGET_DIR", out.join("tapes-target"))
            .status()
            .expect("couldn't run cargo");
        assert!(status.success(), "building {TAPES_CRATE} failed");
        let _ = fs::remove_dir_all(out.join("tapes-target"));
    }
    fs::copy(built, bundled).unwrap();
}

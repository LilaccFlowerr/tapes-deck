use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

/// The tapes release that ships inside tapes-deck. Bump it to ship a newer tapes.
const TAPES_VERSION: &str = "0.1.6-beta.1";

fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    bundle_tapes();
}

/// Puts a `tapes` binary at `$OUT_DIR/tapes` for src/tapes/cli.rs to embed.
/// `TAPES_BINARY=/path/to/tapes` uses a prebuilt one instead of building the crate.
fn bundle_tapes() {
    println!("cargo:rerun-if-env-changed=TAPES_BINARY");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let bundled = out.join("tapes");

    if let Some(prebuilt) = env::var_os("TAPES_BINARY") {
        println!("cargo:rerun-if-changed={}", prebuilt.to_string_lossy());
        fs::copy(prebuilt, bundled).expect("couldn't copy TAPES_BINARY");
        return;
    }

    let root = out.join(format!("tapes-{TAPES_VERSION}"));
    let built = root.join("bin/tapes");
    if !built.is_file() {
        let status = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
            .args(["install", "tapes-v2", "--locked", "--version", &format!("={TAPES_VERSION}")])
            .arg("--root")
            .arg(&root)
            // The outer cargo holds the lock on our own target dir.
            .env("CARGO_TARGET_DIR", out.join("tapes-target"))
            .status()
            .expect("couldn't run cargo");
        assert!(status.success(), "building tapes-v2 failed");
        let _ = fs::remove_dir_all(out.join("tapes-target"));
    }
    fs::copy(built, bundled).unwrap();
}

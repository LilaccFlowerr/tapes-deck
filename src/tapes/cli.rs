use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::types::Output;

const NOT_FOUND: &str = "tapes-v2 not installed";
pub const PAGE: &str = "https://github.com/futurekismo-oss/tapes-v2";

pub fn installed() -> bool {
    bin().is_file()
}

fn bin() -> PathBuf {
    if let Ok(bin) = std::env::var("TAPES_BIN") {
        return bin.into();
    }
    if let Ok(path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join("tapes-v2");
            if candidate.is_file() {
                return candidate;
            }
        }
    }
    std::env::home_dir()
        .map(|home| home.join(".local/bin/tapes-v2"))
        .unwrap_or_else(|| "tapes-v2".into())
}

pub fn run<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    // Hooks may start daemons (`setsid -f ...`) that inherit our stdout/stderr. With pipes,
    // reading the output would wait until those daemons exit, so capture into a file instead.
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("tapesgui-{}-{id}.out", std::process::id()));
    let result = std::fs::File::create(&path).and_then(|file| {
        Command::new(bin())
            .args(args)
            .stdin(Stdio::null())
            .stdout(file.try_clone()?)
            .stderr(file)
            .status()
    });
    let text = std::fs::read(&path).map(|b| String::from_utf8_lossy(&b).into_owned()).unwrap_or_default();
    let _ = std::fs::remove_file(&path);
    match result {
        Ok(status) => Output { ok: status.success(), text },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Output { ok: false, text: NOT_FOUND.into() },
        Err(e) => Output { ok: false, text: format!("couldn't start tapes-v2: {e}") },
    }
}

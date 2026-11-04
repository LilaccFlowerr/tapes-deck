use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::parse;
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
    find_in_path("tapes-v2")
        .or_else(|| std::env::home_dir().map(|home| home.join(".local/bin/tapes-v2")))
        .unwrap_or_else(|| "tapes-v2".into())
}

fn find_in_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path).map(|dir| dir.join(name)).find(|candidate| candidate.is_file())
}

pub fn run<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let args = args.into_iter().map(|a| a.as_ref().to_owned()).collect();
    run_command(bin(), args, |_, _| {})
}

/// Like `run`, but reports the current line of output and, when known, how far along it is.
/// Git only prints percentages to a terminal, so this runs tapes-v2 inside `script`'s when it can.
pub fn run_live<I, S>(args: I, progress: impl FnMut(&str, Option<f32>)) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let args: Vec<std::ffi::OsString> = args.into_iter().map(|a| a.as_ref().to_owned()).collect();
    let Some(script) = find_in_path("script") else {
        return run_command(bin(), args, progress);
    };
    let quote = |s: &std::ffi::OsStr| format!("'{}'", s.to_string_lossy().replace('\'', "'\\''"));
    let command: Vec<String> =
        std::iter::once(bin().into_os_string()).chain(args).map(|a| quote(&a)).collect();
    run_command(script, vec!["-qefc".into(), command.join(" ").into(), "/dev/null".into()], progress)
}

fn run_command(
    program: PathBuf,
    args: Vec<std::ffi::OsString>,
    mut progress: impl FnMut(&str, Option<f32>),
) -> Output {
    // Hooks may start daemons (`setsid -f ...`) that inherit our stdout/stderr. With pipes,
    // reading the output would wait until those daemons exit, so capture into a file instead.
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("tapes-deck-{}-{id}.out", std::process::id()));
    let read = || std::fs::read(&path).map(|b| String::from_utf8_lossy(&b).into_owned()).unwrap_or_default();

    let result = std::fs::File::create(&path).and_then(|file| {
        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::null())
            .stdout(file.try_clone()?)
            .stderr(file)
            .spawn()?;
        let mut last = String::new();
        loop {
            if let Some(status) = child.try_wait()? {
                return Ok(status);
            }
            let tail = read_tail(&path);
            let line = parse::current_line(&tail);
            if line != last {
                last = line.to_string();
                progress(line, parse::parse_fraction(line));
            }
            std::thread::sleep(std::time::Duration::from_millis(80));
        }
    });
    let text = parse::clean_output(&read());
    let _ = std::fs::remove_file(&path);
    match result {
        Ok(status) => Output { ok: status.success(), text },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Output { ok: false, text: NOT_FOUND.into() },
        Err(e) => Output { ok: false, text: format!("couldn't start tapes-v2: {e}") },
    }
}

/// The end of a growing output file; a big clone writes a lot of progress lines.
fn read_tail(path: &std::path::Path) -> String {
    use std::io::{Read, Seek, SeekFrom};
    let Ok(mut file) = std::fs::File::open(path) else { return String::new() };
    let len = file.metadata().map_or(0, |m| m.len());
    let _ = file.seek(SeekFrom::Start(len.saturating_sub(4096)));
    let mut bytes = Vec::new();
    let _ = file.read_to_end(&mut bytes);
    String::from_utf8_lossy(&bytes).into_owned()
}

const INSTALL_SCRIPT: &str =
    "curl -fsSL https://raw.githubusercontent.com/futurekismo-oss/tapes-v2/main/install.sh | bash";

/// Runs the official tapes-v2 install script.
pub fn download() -> Output {
    // The script only falls back to `sudo` when ~/.local/bin isn't in PATH, which a GUI session
    // can't answer. Putting it first keeps the install in the user's own directory.
    let mut path = std::env::var_os("PATH").unwrap_or_default();
    if let Some(home) = std::env::home_dir() {
        let mut dirs = vec![home.join(".local/bin")];
        dirs.extend(std::env::split_paths(&path));
        path = std::env::join_paths(dirs).unwrap_or(path);
    }
    match Command::new("sh").arg("-c").arg(INSTALL_SCRIPT).env("PATH", path).stdin(Stdio::null()).output() {
        Ok(out) => {
            let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&out.stderr));
            // The script exits 0 without installing on NixOS, so check for the binary too.
            let ok = out.status.success() && installed();
            if out.status.success() && !ok {
                text.push_str("\ntapes-v2 was not installed, see the output above");
            }
            Output { ok, text }
        }
        Err(e) => Output { ok: false, text: format!("couldn't start the installer: {e}") },
    }
}

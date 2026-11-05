use std::os::unix::fs::PermissionsExt;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::parse;
use super::types::Output;

const NOT_FOUND: &str = "tapes not installed";

/// The tapes binary built into tapes-deck, see build.rs.
const BUNDLED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tapes"));

fn installed() -> bool {
    bin().is_file()
}

fn bin() -> PathBuf {
    if let Ok(bin) = std::env::var("TAPES_BIN") {
        return bin.into();
    }
    find_in_path("tapes")
        .or_else(|| std::env::home_dir().map(|home| home.join(".local/bin/tapes")))
        .unwrap_or_else(|| "tapes".into())
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
    run_command(bin(), args, None, |_, _| {})
}

/// Like `run`, but reports the current line of output and, when known, how far along it is.
/// Git only prints percentages to a terminal, so this runs tapes inside `script`'s when it can.
pub fn run_live<I, S>(args: I, progress: impl FnMut(&str, Option<f32>)) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let args = args.into_iter().map(|a| a.as_ref().to_owned()).collect();
    let (program, args) = in_terminal(bin(), args);
    run_command(program, args, None, progress)
}

/// Runs a shell script in `cwd`, reporting its output the same way as `run_live`.
pub fn run_script(script: &str, cwd: &Path, progress: impl FnMut(&str, Option<f32>)) -> Output {
    let (program, args) = in_terminal("sh".into(), vec!["-c".into(), script.into()]);
    run_command(program, args, Some(cwd), progress)
}

fn in_terminal(program: PathBuf, args: Vec<OsString>) -> (PathBuf, Vec<OsString>) {
    let Some(script) = find_in_path("script") else { return (program, args) };
    let quote = |s: &std::ffi::OsStr| format!("'{}'", s.to_string_lossy().replace('\'', "'\\''"));
    let command: Vec<String> = std::iter::once(program.into_os_string()).chain(args).map(|a| quote(&a)).collect();
    (script, vec!["-qefc".into(), command.join(" ").into(), "/dev/null".into()])
}

fn run_command(
    program: PathBuf,
    args: Vec<OsString>,
    cwd: Option<&Path>,
    mut progress: impl FnMut(&str, Option<f32>),
) -> Output {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("tapes-deck-{}-{id}.out", std::process::id()));
    let read = || std::fs::read(&path).map(|b| String::from_utf8_lossy(&b).into_owned()).unwrap_or_default();

    let result = std::fs::File::create(&path).and_then(|file| {
        let mut command = Command::new(program);
        command.args(args).stdin(Stdio::null()).stdout(file.try_clone()?).stderr(file);
        if let Some(cwd) = cwd {
            command.current_dir(cwd);
        }
        let mut child = command.spawn()?;
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
        Err(e) => Output { ok: false, text: format!("couldn't start tapes: {e}") },
    }
}

fn read_tail(path: &std::path::Path) -> String {
    use std::io::{Read, Seek, SeekFrom};
    let Ok(mut file) = std::fs::File::open(path) else { return String::new() };
    let len = file.metadata().map_or(0, |m| m.len());
    let _ = file.seek(SeekFrom::Start(len.saturating_sub(4096)));
    let mut bytes = Vec::new();
    let _ = file.read_to_end(&mut bytes);
    String::from_utf8_lossy(&bytes).into_owned()
}

pub fn ensure() -> std::io::Result<()> {
    if installed() {
        return Ok(());
    }
    let target = bin();
    let dir = target.parent().ok_or(std::io::ErrorKind::NotFound)?;
    std::fs::create_dir_all(dir)?;
    let partial = dir.join(format!(".tapes.{}", std::process::id()));
    std::fs::write(&partial, BUNDLED)?;
    std::fs::set_permissions(&partial, std::fs::Permissions::from_mode(0o755))?;
    std::fs::rename(&partial, &target)
}

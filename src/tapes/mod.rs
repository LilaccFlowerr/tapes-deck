mod cli;
mod config;
mod explain;
mod parse;
mod types;

pub use cli::ensure;
pub use config::{load as load_config, save as save_config, Config};
pub use explain::describe as describe_hook;
pub use types::{Details, Options, Output, Tape};

use cli::{run, run_install};
pub use parse::{error, installed_tape};
use parse::{parse_active, parse_details, parse_list};

fn with_options(mut args: Vec<String>, options: Options) -> Vec<String> {
    if options.dry_run {
        args.push("--dry-run".into());
    }
    if options.no_hooks {
        args.push("--no-hooks".into());
    }
    args
}

pub fn list() -> Vec<Tape> {
    parse_list(&run(["show"]).text)
}

pub fn details(name: &str) -> Details {
    parse_details(&run(["show", name]).text)
}

pub fn active(tapes: &[Tape]) -> Option<usize> {
    parse_active(&run(["status"]).text, tapes)
}

pub fn insert(name: &str, options: Options) -> Output {
    run(with_options(vec!["insert".into(), name.into(), "--yes".into()], options))
}

#[allow(dead_code)]
pub fn insert_path(path: &str, options: Options) -> Output {
    run(with_options(vec!["insert".into(), "--path".into(), path.into(), "--yes".into()], options))
}

pub fn eject(options: Options) -> Output {
    run(with_options(vec!["eject".into()], options))
}

pub fn install(
    source: &str,
    name: Option<&str>,
    branch: Option<&str>,
    dir: Option<&str>,
    progress: impl FnMut(&str, Option<f32>),
) -> Output {
    run_install(install_args(source, name.unwrap_or(""), branch.unwrap_or(""), dir.unwrap_or("")), progress)
}

const FEATURED_REPO: &str = "https://github.com/futurekismo-oss/featured-tapes";

pub fn featured() -> Vec<(String, Config)> {
    let Some(dir) = std::env::home_dir().map(|home| home.join(".cache/tapes-deck/featured-tapes")) else {
        return Vec::new();
    };
    let _ = std::fs::remove_dir_all(&dir);
    let cloned = std::process::Command::new("git")
        .args(["clone", "--depth", "1", "--quiet", FEATURED_REPO])
        .arg(&dir)
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if !cloned {
        return Vec::new();
    }
    let mut tapes: Vec<(String, Config)> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let text = std::fs::read_to_string(entry.path().join("tape.toml")).ok()?;
            Some((entry.file_name().into_string().ok()?, config::parse(&text).unwrap_or_default()))
        })
        .collect();
    tapes.sort_by(|a, b| a.0.cmp(&b.0));
    tapes
}

pub fn install_featured(name: &str, progress: impl FnMut(&str, Option<f32>)) -> Output {
    run_install(vec!["install-featured".into(), name.into(), "--yes".into()], progress)
}

fn install_args(source: &str, name: &str, branch: &str, dir: &str) -> Vec<String> {
    let mut args = vec!["install".to_string(), source.to_string(), "--yes".to_string()];
    for (flag, value) in [("--name", name), ("--branch", branch), ("--dir", dir)] {
        if !value.is_empty() {
            args.push(flag.into());
            args.push(value.into());
        }
    }
    args
}


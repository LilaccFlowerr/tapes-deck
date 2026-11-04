mod cli;
mod parse;
mod types;

pub use cli::{download as install_cli, installed, PAGE};
pub use types::{Details, Options, Output, Tape};

use cli::run;
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
    run(with_options(vec!["insert".into(), name.into()], options))
}

// Not wired into the UI yet.
#[allow(dead_code)]
pub fn insert_path(path: &str, options: Options) -> Output {
    run(with_options(vec!["insert".into(), "--path".into(), path.into()], options))
}

pub fn eject(options: Options) -> Output {
    run(with_options(vec!["eject".into()], options))
}

pub fn install(source: &str, name: Option<&str>, branch: Option<&str>, dir: Option<&str>) -> Output {
    let mut args = vec!["install".to_string(), source.to_string()];
    for (flag, value) in [("--name", name), ("--branch", branch), ("--dir", dir)] {
        if let Some(value) = value.filter(|v| !v.is_empty()) {
            args.push(flag.into());
            args.push(value.into());
        }
    }
    run(args)
}

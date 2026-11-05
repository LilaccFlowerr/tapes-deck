mod cli;
mod config;
mod explain;
mod featured;
mod parse;
mod types;

pub use cli::ensure;
pub use config::{load as load_config, save as save_config, Config};
pub use explain::describe as describe_hook;
pub use featured::FEATURED;
use featured::Featured;
pub use types::{Details, Options, Output, Tape};

use cli::{run, run_live, run_script};
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

pub fn install(
    source: &str,
    name: Option<&str>,
    branch: Option<&str>,
    dir: Option<&str>,
    progress: impl FnMut(&str, Option<f32>),
) -> Output {
    let (name, branch, dir) = (name.unwrap_or(""), branch.unwrap_or(""), dir.unwrap_or(""));
    let featured = FEATURED.iter().find(|tape| tape.source == source && tape.dir == dir && branch.is_empty());
    match featured {
        Some(tape) => install_featured(tape, name, progress),
        None => run_live(install_args(source, name, branch, dir, false), progress),
    }
}

fn install_args(source: &str, name: &str, branch: &str, dir: &str, no_hooks: bool) -> Vec<String> {
    let mut args = vec!["install".to_string(), source.to_string()];
    for (flag, value) in [("--name", name), ("--branch", branch), ("--dir", dir)] {
        if !value.is_empty() {
            args.push(flag.into());
            args.push(value.into());
        }
    }
    if no_hooks {
        args.push("--no-hooks".into());
    }
    args
}

fn install_featured(tape: &Featured, name: &str, mut progress: impl FnMut(&str, Option<f32>)) -> Output {
    let name = if name.is_empty() { tape.name } else { name }.to_lowercase();
    let steps = if tape.setup.is_some() { 2.0 } else { 1.0 };
    let downloading = format!("Downloading {} from GitHub", tape.name);
    let mut out = run_live(install_args(tape.source, &name, "", tape.dir, true), |line, fraction| {
        progress(&format!("{downloading}\n{line}"), fraction.map(|f| f / steps));
    });
    if !out.ok {
        return out;
    }

    let Some(folder) = std::env::home_dir().map(|home| home.join(".tapes").join(&name)) else {
        return Output { ok: false, text: "couldn't find your home folder".into() };
    };
    if let Err(e) = std::fs::write(folder.join("tape.toml"), tape.tape_toml.replace("@NAME@", &name)) {
        return Output { ok: false, text: format!("couldn't write tape.toml: {e}") };
    }

    if let Some(script) = tape.setup {
        let mut stage = String::from("Setting things up");
        progress(&stage, Some(1.0 / steps));
        let setup = run_script(script, &folder, |line, _| match line.strip_prefix("==> ") {
            Some(text) => {
                stage = text.to_string();
                progress(&stage, None);
            }
            None => progress(&format!("{stage}\n{line}"), None),
        });
        out.text.push('\n');
        out.text.push_str(&setup.text);
        if !setup.ok {
            out.ok = false;
            out.text.push_str("\nsetup failed: the tape is installed but may not work yet");
        }
    }
    out
}

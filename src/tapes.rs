#![allow(dead_code)]

use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, PartialEq)]
pub struct Tape {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Details {
    pub description: String,
    pub insert_hooks: Vec<String>,
    pub eject_hooks: Vec<String>,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    pub dry_run: bool,
    pub no_hooks: bool,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub ok: bool,
    pub text: String,
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

fn run<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    match Command::new(bin()).args(args).stdin(Stdio::null()).output() {
        Ok(out) => {
            let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&out.stderr));
            Output { ok: out.status.success(), text }
        }
        Err(e) => Output { ok: false, text: format!("couldn't start tapes-v2: {e}") },
    }
}

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

fn parse_list(text: &str) -> Vec<Tape> {
    text.lines()
        .filter_map(|line| line.trim().strip_prefix("- "))
        .filter_map(|line| line.split_once(": "))
        .map(|(name, path)| Tape { name: name.trim().into(), path: path.trim().into() })
        .collect()
}

fn parse_details(text: &str) -> Details {
    enum Section {
        None,
        Insert,
        Eject,
        Targets,
    }
    let mut details = Details::default();
    let mut section = Section::None;
    for line in text.lines().map(str::trim) {
        if let Some(description) = line.strip_prefix("Description:") {
            details.description = description.trim().into();
        } else if line == "Insert:" {
            section = Section::Insert;
        } else if line == "Eject:" {
            section = Section::Eject;
        } else if line == "Targets" {
            section = Section::Targets;
        } else if let Some(item) = line.strip_prefix("- ") {
            match section {
                Section::Insert => details.insert_hooks.push(item.into()),
                Section::Eject => details.eject_hooks.push(item.into()),
                Section::Targets => details.targets.push(item.into()),
                Section::None => {}
            }
        }
    }
    details
}

fn parse_active(text: &str, tapes: &[Tape]) -> Option<usize> {
    if text.contains("No Tapes is currently inserted") {
        return None;
    }
    let words: Vec<&str> = text
        .split(|c: char| !(c.is_alphanumeric() || matches!(c, '-' | '_' | '.')))
        .collect();
    tapes.iter().position(|tape| words.contains(&tape.name.as_str()))
}

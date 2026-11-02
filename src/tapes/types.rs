use std::path::PathBuf;

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

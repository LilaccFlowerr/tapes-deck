use std::path::PathBuf;

use toml_edit::{Array, DocumentMut, Item, Table, Value};
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Config {
    pub name: String,
    pub desc: String,
    pub insert: Vec<String>,
    pub eject: Vec<String>,
    pub install: Vec<String>,
}

fn path(tape: &str) -> Option<PathBuf> {
    std::env::home_dir().map(|home| home.join(".tapes").join(tape.to_lowercase()).join("tape.toml"))
}

pub fn load(tape: &str) -> Result<Config, String> {
    let path = path(tape).ok_or("couldn't find your home folder")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("couldn't read {}: {e}", path.display()))?;
    parse(&text)
}

pub fn save(tape: &str, config: &Config) -> Result<(), String> {
    let path = path(tape).ok_or("couldn't find your home folder")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("couldn't read {}: {e}", path.display()))?;
    let text = render(&text, config)?;
    std::fs::write(&path, text).map_err(|e| format!("couldn't write {}: {e}", path.display()))
}

fn parse(text: &str) -> Result<Config, String> {
    let doc: DocumentMut = text.parse().map_err(|e| format!("tape.toml is not valid: {e}"))?;
    let tape = doc.get("tape").and_then(Item::as_table);
    let text_of = |key: &str| {
        tape.and_then(|t| t.get(key)).and_then(Item::as_str).unwrap_or_default().to_string()
    };
    let hooks = doc.get("hooks").and_then(Item::as_table);
    let list = |key: &str| -> Vec<String> {
        hooks
            .and_then(|h| h.get(key))
            .and_then(Item::as_array)
            .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
            .unwrap_or_default()
    };
    Ok(Config {
        name: text_of("name"),
        desc: text_of("desc"),
        insert: list("insert"),
        eject: list("eject"),
        install: list("install"),
    })
}

fn render(text: &str, config: &Config) -> Result<String, String> {
    let mut doc: DocumentMut = text.parse().map_err(|e| format!("tape.toml is not valid: {e}"))?;

    let tape = table(&mut doc, "tape")?;
    tape["name"] = toml_edit::value(config.name.trim());
    tape["desc"] = toml_edit::value(config.desc.trim());

    let hooks = table(&mut doc, "hooks")?;
    for (key, lines) in [("insert", &config.insert), ("eject", &config.eject), ("install", &config.install)] {
        if lines.iter().all(|l| l.trim().is_empty()) && !hooks.contains_key(key) {
            continue;
        }
        hooks[key] = Item::Value(Value::Array(hook_array(lines)));
    }
    Ok(doc.to_string())
}

fn table<'a>(doc: &'a mut DocumentMut, key: &str) -> Result<&'a mut Table, String> {
    if !doc.contains_key(key) {
        doc[key] = Item::Table(Table::new());
    }
    doc[key].as_table_mut().ok_or_else(|| format!("[{key}] in tape.toml is not a table"))
}

fn hook_array(lines: &[String]) -> Array {
    let mut array = Array::new();
    for line in lines.iter().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let mut value = Value::from(line);
        value.decor_mut().set_prefix("\n    ");
        array.push_formatted(value);
    }
    array.set_trailing("\n");
    array.set_trailing_comma(true);
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"[tape]
name = "x"
desc = "A tape"
dependencies = ["hyprland", "git"]

[hooks]
# comment that must survive
insert = [
    "hyprctl reload",
]
eject = []
"#;

    #[test]
    fn reads_the_fields() {
        let config = parse(SAMPLE).unwrap();
        assert_eq!(config.name, "x");
        assert_eq!(config.desc, "A tape");
        assert_eq!(config.insert, ["hyprctl reload"]);
        assert!(config.eject.is_empty() && config.install.is_empty());
    }

    #[test]
    fn saving_changes_only_what_was_edited() {
        let mut config = parse(SAMPLE).unwrap();
        config.desc = "New".into();
        config.insert.push("pkill -x qs || true".into());
        config.install.push("echo hi".into());
        let out = render(SAMPLE, &config).unwrap();
        assert!(out.contains("# comment that must survive"));
        assert!(out.contains(r#"dependencies = ["hyprland", "git"]"#));
        assert_eq!(parse(&out).unwrap(), config);
    }

    #[test]
    fn hooks_with_quotes_survive_a_round_trip() {
        let mut config = parse(SAMPLE).unwrap();
        config.eject = vec![r#"sh -c 'test -f "$f" || true'"#.into()];
        assert_eq!(parse(&render(SAMPLE, &config).unwrap()).unwrap(), config);
    }

    #[test]
    fn empty_lines_are_dropped_and_missing_tables_are_made() {
        let config = Config { name: "y".into(), desc: "d".into(), insert: vec!["  ".into(), "a".into()], ..Config::default() };
        let out = render("", &config).unwrap();
        assert_eq!(parse(&out).unwrap().insert, ["a"]);
        assert!(!out.contains("eject"));
    }

    #[test]
    fn broken_toml_is_an_error_not_a_panic() {
        assert!(parse("[tape\nname =").is_err());
        assert!(render("[tape\nname =", &Config::default()).is_err());
    }
}

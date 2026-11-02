use super::types::{Details, Tape};

pub fn parse_list(text: &str) -> Vec<Tape> {
    text.lines()
        .filter_map(|line| line.trim().strip_prefix("- "))
        .filter_map(|line| line.split_once(": "))
        .map(|(name, path)| Tape { name: name.trim().into(), path: path.trim().into() })
        .collect()
}

pub fn parse_details(text: &str) -> Details {
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

pub fn parse_active(text: &str, tapes: &[Tape]) -> Option<usize> {
    if text.contains("No Tapes is currently inserted") {
        return None;
    }
    let words: Vec<&str> = text
        .split(|c: char| !(c.is_alphanumeric() || matches!(c, '-' | '_' | '.')))
        .collect();
    tapes.iter().position(|tape| words.contains(&tape.name.as_str()))
}

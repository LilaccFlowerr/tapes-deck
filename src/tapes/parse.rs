use super::types::{Details, Tape};

/// What is on screen right now in live output: git rewrites its progress line with `\r`.
pub fn current_line(text: &str) -> &str {
    text.split(['\r', '\n'])
        .map(str::trim)
        .rfind(|line| !line.is_empty())
        .unwrap_or("")
        .trim_start_matches("remote: ")
}

/// Overall progress (0..=1) of a git clone, read from a line like `Receiving objects:  45% (318/707)`.
pub fn parse_fraction(line: &str) -> Option<f32> {
    let (start, end) = match line.split(':').next()? {
        "Receiving objects" => (0.0, 0.8),
        "Resolving deltas" => (0.8, 0.95),
        "Updating files" => (0.95, 1.0),
        _ => return None,
    };
    let before = &line[..line.find('%')?];
    let digits = before.rsplit(' ').next()?;
    let percent: f32 = digits.parse().ok()?;
    Some(start + (end - start) * percent / 100.0)
}

/// Collapses progress lines that were redrawn in place to the version that ended up on screen.
pub fn clean_output(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end_matches('\r').rsplit('\r').next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_line_git_is_redrawing() {
        let live = "Cloning into 'x'...\r\nReceiving objects:  44% (312/707)\rReceiving objects:  45% (319/707), 42 MiB | 4 MiB/s";
        assert_eq!(current_line(live), "Receiving objects:  45% (319/707), 42 MiB | 4 MiB/s");
        assert_eq!(current_line("remote: Counting objects: 3% (1/30)\r"), "Counting objects: 3% (1/30)");
    }

    #[test]
    fn maps_phases_onto_one_bar() {
        assert_eq!(parse_fraction("Receiving objects:   0% (1/707)"), Some(0.0));
        assert_eq!(parse_fraction("Receiving objects: 50% (354/707), 4 MiB | 1 MiB/s"), Some(0.4));
        assert_eq!(parse_fraction("Resolving deltas: 100% (429/429), done."), Some(0.95));
        assert!((parse_fraction("Updating files:  50% (5/10)").unwrap() - 0.975).abs() < 1e-4);
        assert_eq!(parse_fraction("Cloning into 'x'..."), None);
        assert_eq!(parse_fraction("Counting objects: 50% (1/2)"), None);
    }

    #[test]
    fn cleans_redrawn_output() {
        let raw = "Cloning into 'x'...\r\nReceiving objects:  1% (1/9)\rReceiving objects: 100% (9/9), done.\r\nDone\r\n";
        assert_eq!(clean_output(raw), "Cloning into 'x'...\nReceiving objects: 100% (9/9), done.\nDone");
    }
}

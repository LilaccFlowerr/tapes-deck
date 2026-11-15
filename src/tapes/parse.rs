use super::types::{Details, Tape};

pub fn strip_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\u{1b}' {
            out.push(c);
        } else if chars.peek() == Some(&'[') {
            chars.next();
            for end in chars.by_ref() {
                if ('@'..='~').contains(&end) {
                    break;
                }
            }
        }
    }
    out
}

pub fn current_line(text: &str) -> String {
    let text = strip_ansi(text);
    text.split(['\r', '\n'])
        .map(str::trim)
        .rfind(|line| !line.is_empty())
        .unwrap_or("")
        .trim_start_matches("remote: ")
        .to_string()
}

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

pub fn clean_output(text: &str) -> String {
    strip_ansi(text)
        .lines()
        .map(|line| line.trim_end_matches('\r').rsplit('\r').next().unwrap_or("").to_string())
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

pub fn installed_tape(text: &str) -> Option<String> {
    let rest = text.split("Successfully installed tape '").nth(1)?;
    Some(rest.split('\'').next()?.to_string())
}

pub fn error(text: &str) -> String {
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|line| !line.is_empty()).collect();
    match lines.iter().rposition(|line| line.starts_with("Error:")) {
        Some(at) => lines[at..].join("\n"),
        None => lines.last().unwrap_or(&"").to_string(),
    }
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
    let lower = text.to_lowercase();
    if lower.contains("no tapes is currently inserted") || lower.contains("no tape is currently inserted") {
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
    fn colour_codes_do_not_reach_the_screen() {
        assert_eq!(strip_ansi("\u{1b}[1m\u{1b}[92m   Compiling\u{1b}[0m futures v0.3.32"), "   Compiling futures v0.3.32");
        assert_eq!(current_line("\u{1b}[1m\u{1b}[92m   Compiling\u{1b}[0m futures v0.3.32\r\n"), "Compiling futures v0.3.32");
        assert_eq!(strip_ansi("plain text"), "plain text");
        assert_eq!(clean_output("\u{1b}[31mred\u{1b}[0m"), "red");
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

    #[test]
    fn takes_what_tapes_says() {
        let text = "Copying tape 'end-4'\nError: Tape 'end-4' already exists at /home/me/.tapes/end-4.\nUse --force to overwrite.\n";
        assert_eq!(error(text), "Error: Tape 'end-4' already exists at /home/me/.tapes/end-4.\nUse --force to overwrite.");
        assert_eq!(error("cloning\nfatal: repository not found\n"), "fatal: repository not found");
        assert_eq!(installed_tape("Successfully installed tape 'end-4' to /home/me/.tapes/end-4").as_deref(), Some("end-4"));
    }
}

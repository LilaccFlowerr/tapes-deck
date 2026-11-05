pub fn describe(command: &str) -> String {
    let command = command.trim();
    let mut words = command.split_whitespace();
    let program = words.next().unwrap_or("");
    let rest: Vec<&str> = words.collect();

    match program {
        "hyprctl" if rest.first() == Some(&"reload") => "Reload Hyprland so it picks up the config".to_string(),
        "hyprctl" => "Send a command to Hyprland".to_string(),
        "pkill" | "killall" => format!("Stop {}", stop_target(&rest)),
        "qs" | "quickshell" => match config_name(&rest) {
            Some(name) => format!("Start the {name} shell in the background"),
            None => "Start a shell in the background".to_string(),
        },
        "caelestia" if rest.first() == Some(&"shell") => "Start the caelestia shell in the background".to_string(),
        "caelestia" if rest.first() == Some(&"scheme") => "Set the colours".to_string(),
        "notify-send" => "Show a notification".to_string(),
        "mkdir" => "Make a folder".to_string(),
        "touch" => "Make an empty file if it isn't there".to_string(),
        "sh" | "bash" if command.contains("caelestia scheme") => "Pick the default colours if none are set yet".to_string(),
        "sh" | "bash" => "Run a small script".to_string(),
        "" => String::new(),
        _ if command.ends_with('&') => "Start a program in the background".to_string(),
        _ => String::new(),
    }
}
fn stop_target(rest: &[&str]) -> String {
    match rest.iter().rfind(|w| !w.starts_with('-') && **w != "||" && **w != "true") {
        Some(&"qs") | Some(&"quickshell") => "the shell that is running".to_string(),
        Some(name) => name.to_string(),
        None => "a running program".to_string(),
    }
}
fn config_name(rest: &[&str]) -> Option<String> {
    let at = rest.iter().position(|w| *w == "-c" || *w == "--config")?;
    rest.get(at + 1).map(|name| name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explains_the_commands_the_featured_tapes_use() {
        assert_eq!(describe("hyprctl reload"), "Reload Hyprland so it picks up the config");
        assert_eq!(describe("pkill -x qs || true"), "Stop the shell that is running");
        assert_eq!(describe("qs -c nebula -d"), "Start the nebula shell in the background");
        assert_eq!(describe("caelestia shell -d >/dev/null"), "Start the caelestia shell in the background");
        assert_eq!(
            describe("sh -c 'test -f ~/.local/state/caelestia/scheme.json || caelestia scheme set -n caelestia'"),
            "Pick the default colours if none are set yet"
        );
    }

    #[test]
    fn other_programs_and_unknown_commands() {
        assert_eq!(describe("pkill waybar"), "Stop waybar");
        assert_eq!(describe("waybar &"), "Start a program in the background");
        assert_eq!(describe("notify-send 'Tapes' 'hi'"), "Show a notification");
        assert_eq!(describe("my-own-thing --flag"), "");
        assert_eq!(describe(""), "");
    }
}

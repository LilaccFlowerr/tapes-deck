pub struct Tutorial {
    pub about: &'static str,
    pub insert: &'static str,
    pub eject: &'static str,
    pub finish: &'static str,
}

pub struct Featured {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
    pub dir: &'static str,
    pub tape_toml: &'static str,
    pub setup: Option<&'static str>,
    pub tutorial: Tutorial,
}

pub const FEATURED: &[Featured] = &[
    Featured {
        name: "caelestia",
        description: "the caelestia dots, default settings",
        source: "https://github.com/caelestia-dots/caelestia",
        dir: "",
        tape_toml: include_str!("../../featured/caelestia/tape.toml"),
        setup: Some(include_str!("../../featured/caelestia/setup.sh")),
        tutorial: Tutorial {
            about: "caelestia is a desktop shell with a bar, launcher and widgets whose colours follow your wallpaper. This tape holds its dots (hypr, fish, foot and more) and the shell itself, so inserting it swaps your whole setup for caelestia.",
            insert: "Here Hyprland reloads with caelestia's config, the shell that was running is stopped and caelestia's own shell starts. Keep these three in this order.",
            eject: "On eject your own config is back and caelestia's shell is stopped. Nothing starts your own shell again unless you add it here.",
            finish: "caelestia needs packages this app doesn't install: see dependencies in this tape's tape.toml. Its shell was built for you if it wasn't there yet.",
        },
    },
    Featured {
        name: "nebula",
        description: "a Quickshell desktop made of shapes",
        source: "https://github.com/LilaccFlowerr/Nebula",
        dir: "",
        tape_toml: include_str!("../../featured/nebula/tape.toml"),
        setup: Some(include_str!("../../featured/nebula/setup.sh")),
        tutorial: Tutorial {
            about: "Nebula is a Hyprland desktop built around a Quickshell shell, with Material 3 shapes and colours that follow your wallpaper.",
            insert: "Here Hyprland reloads with Nebula's config, the shell that was running is stopped and Nebula's own shell starts (qs -c nebula).",
            eject: "On eject your own config is back and Nebula's shell is stopped. Nothing starts your own shell again unless you add it here.",
            finish: "Nebula needs packages this app doesn't install: see dependencies in this tape's tape.toml. M3Shapes was built for you if it wasn't there yet.",
        },
    },
    Featured {
        name: "end-4",
        description: "illogical-impulse by end-4",
        source: "https://github.com/end-4/dots-hyprland",
        dir: "dots/.config",
        tape_toml: include_str!("../../featured/end-4/tape.toml"),
        setup: None,
        tutorial: Tutorial {
            about: "These are end-4's dots (illogical-impulse): only the config folders, none of the programs they use.",
            insert: "Here Hyprland reloads with end-4's config, the shell that was running is stopped and end-4's shell starts (qs -c ii).",
            eject: "On eject your own config is back and end-4's shell is stopped. Nothing starts your own shell again unless you add it here.",
            finish: "end-4's setup needs a lot of packages that this app doesn't install: see dependencies in this tape's tape.toml, and end-4's own install guide for the full list.",
        },
    },
];

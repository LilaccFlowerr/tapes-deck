/// A ready-made tape offered on the install page. Picking one only fills in the form; installing
/// it clones the dots' own repo, writes the `tape.toml` below and runs the setup script once.
pub struct Featured {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
    pub dir: &'static str,
    /// The tape.toml to write, with `@NAME@` standing in for the name the tape was installed under.
    pub tape_toml: &'static str,
    /// Run in the tape's folder after installing, for dots that need something built first.
    pub setup: Option<&'static str>,
}

pub const FEATURED: &[Featured] = &[
    Featured {
        name: "caelestia",
        description: "the caelestia dots, default settings",
        source: "https://github.com/caelestia-dots/caelestia",
        dir: "",
        tape_toml: include_str!("../../featured/caelestia/tape.toml"),
        setup: Some(include_str!("../../featured/caelestia/setup.sh")),
    },
    Featured {
        name: "nebula",
        description: "a Quickshell desktop made of shapes",
        source: "https://github.com/LilaccFlowerr/Nebula",
        dir: "",
        tape_toml: include_str!("../../featured/nebula/tape.toml"),
        setup: Some(include_str!("../../featured/nebula/setup.sh")),
    },
    Featured {
        name: "end-4",
        description: "illogical-impulse by end-4",
        source: "https://github.com/end-4/dots-hyprland",
        dir: "dots/.config",
        tape_toml: include_str!("../../featured/end-4/tape.toml"),
        setup: None,
    },
];

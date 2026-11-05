pub struct Featured {
    pub name: &'static str,
    pub description: &'static str,
    pub source: &'static str,
    pub dir: &'static str,
    pub tape_toml: &'static str,
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

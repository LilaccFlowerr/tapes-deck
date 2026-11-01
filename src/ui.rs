use std::rc::Rc;

use slint::{ModelRc, VecModel};

use crate::{tapes, AppWindow, TapeInfo};

pub fn load_tapes(ui: &AppWindow) {
    let infos: Vec<TapeInfo> = tapes::list()
        .iter()
        .map(|tape| TapeInfo {
            name: tape.name.as_str().into(),
            source: source(&tapes::details(&tape.name).description).into(),
        })
        .collect();

    ui.set_tapes(ModelRc::from(Rc::new(VecModel::from(infos))));
}

fn source(description: &str) -> &str {
    description
        .strip_prefix("Tape installed from ")
        .unwrap_or(description)
        .trim_start_matches("https://")
}
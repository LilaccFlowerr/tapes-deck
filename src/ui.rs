use std::rc::Rc;

use slint::{ModelRc, VecModel};

use crate::{tapes, AppWindow, TapeInfo};

pub fn load_tapes(ui: &AppWindow) {
    let list = tapes::list();

    let infos: Vec<TapeInfo> = list
        .iter()
        .map(|tape| TapeInfo {
            name: tape.name.as_str().into(),
            source: source(&tapes::details(&tape.name).description).into(),
        })
        .collect();

    ui.set_tapes(ModelRc::from(Rc::new(VecModel::from(infos))));

    let active = tapes::active(&list);
    ui.set_active(active.map_or(-1, |i| i as i32));
}

fn source(description: &str) -> &str {
    description
        .strip_prefix("Tape installed from ")
        .unwrap_or(description)
        .trim_start_matches("https://")
}
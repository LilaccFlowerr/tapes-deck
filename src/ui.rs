use std::rc::Rc;

use slint::{ModelRc, SharedString, VecModel};

use crate::{tapes, AppWindow, TapeInfo};

pub fn load_tapes(ui: &AppWindow) {
    let list = tapes::list();
    let details: Vec<tapes::Details> = list.iter().map(|tape| tapes::details(&tape.name)).collect();

    let infos: Vec<TapeInfo> = list
        .iter()
        .zip(&details)
        .map(|(tape, details)| TapeInfo {
            name: tape.name.as_str().into(),
            source: source(&details.description).into(),
        })
        .collect();

    ui.set_tapes(ModelRc::from(Rc::new(VecModel::from(infos))));

    let active = tapes::active(&list);
    ui.set_active(active.map_or(-1, |i| i as i32));

    let active_details = active.map(|i| details[i].clone()).unwrap_or_default();
    ui.set_active_insert_hooks(strings(active_details.insert_hooks));
    ui.set_active_eject_hooks(strings(active_details.eject_hooks));
    ui.set_active_targets(strings(active_details.targets));
}

fn strings(items: Vec<String>) -> ModelRc<SharedString> {
    let items: Vec<SharedString> = items.into_iter().map(SharedString::from).collect();
    ModelRc::from(Rc::new(VecModel::from(items)))
}

fn source(description: &str) -> &str {
    description
        .strip_prefix("Tape installed from ")
        .unwrap_or(description)
        .trim_start_matches("https://")
}

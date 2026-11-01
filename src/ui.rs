use std::rc::Rc;

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};

use crate::{tapes, AppWindow, TapeInfo};

pub fn setup(ui: &AppWindow) {
    load_tapes(ui);

    let weak = ui.as_weak();
    ui.on_eject(move || {
        run_in_background(&weak, || tapes::eject(tapes::Options::default()));
    });

    let weak = ui.as_weak();
    ui.on_insert(move |index| {
        let Some(ui) = weak.upgrade() else { return };
        let Some(tape) = ui.get_tapes().row_data(index as usize) else { return };
        let name = tape.name.to_string();
        run_in_background(&weak, move || tapes::insert(&name, tapes::Options::default()));
    });
}

fn run_in_background(weak: &Weak<AppWindow>, job: impl FnOnce() -> tapes::Output + Send + 'static) {
    let Some(ui) = weak.upgrade() else { return };
    ui.set_busy(true);
    ui.set_status("".into());

    let weak = weak.clone();
    std::thread::spawn(move || {
        let output = job();
        let _ = weak.upgrade_in_event_loop(move |ui| {
            ui.set_busy(false);
            if !output.ok {
                ui.set_status(last_line(&output.text).into());
            }
            load_tapes(&ui);
        });
    });
}

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

fn last_line(text: &str) -> &str {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .last()
        .unwrap_or("something went wrong")
}

fn source(description: &str) -> &str {
    description
        .strip_prefix("Tape installed from ")
        .unwrap_or(description)
        .trim_start_matches("https://")
}

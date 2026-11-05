use std::rc::Rc;
use std::time::Duration;

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};

use crate::{tapes, AppWindow, FeaturedTape, TapeInfo};

pub fn setup(ui: &AppWindow) {
    if let Err(e) = tapes::ensure() {
        eprintln!("couldn't unpack the bundled tapes: {e}");
    }
    load_tapes(ui);

    let featured: Vec<FeaturedTape> = tapes::FEATURED
        .iter()
        .map(|tape| FeaturedTape {
            name: tape.name.into(),
            description: tape.description.into(),
            source: tape.source.into(),
            branch: "".into(),
            dir: tape.dir.into(),
        })
        .collect();
    ui.set_featured(ModelRc::from(Rc::new(VecModel::from(featured))));

    let weak = ui.as_weak();
    ui.on_install(move |source, name, branch, dir| {
        let progress = weak.clone();
        run_in_background(&weak, Duration::ZERO, move || {
            tapes::install(&source, Some(&name), Some(&branch), Some(&dir), |line, fraction| {
                let line = SharedString::from(line);
                let _ = progress.upgrade_in_event_loop(move |ui| {
                    ui.set_progress(line);
                    if let Some(fraction) = fraction {
                        ui.set_progress_value(fraction);
                    }
                });
            })
        });
    });

    let weak = ui.as_weak();
    ui.on_eject(move || {
        run_in_background(&weak, EJECT_ANIMATION, || tapes::eject(tapes::Options::default()));
    });

    let weak = ui.as_weak();
    ui.on_insert(move |index| {
        let Some(ui) = weak.upgrade() else { return };
        let Some(tape) = ui.get_tapes().row_data(index as usize) else { return };
        let name = tape.name.to_string();
        run_in_background(&weak, INSERT_ANIMATION, move || {
            tapes::insert(&name, tapes::Options::default())
        });
    });
}

// Running a tape starts hooks that load the CPU (a new shell, for one). Waiting until the matching
// animation in the UI has played keeps it smooth instead of stuttering halfway through.
const EJECT_ANIMATION: Duration = Duration::from_millis(650);
const INSERT_ANIMATION: Duration = Duration::from_millis(380);

fn run_in_background(
    weak: &Weak<AppWindow>,
    animation: Duration,
    job: impl FnOnce() -> tapes::Output + Send + 'static,
) {
    let Some(ui) = weak.upgrade() else { return };
    ui.set_busy(true);
    ui.set_status("".into());
    ui.set_progress("".into());
    ui.set_progress_value(0.0);

    let weak = weak.clone();
    std::thread::spawn(move || {
        std::thread::sleep(animation);
        let output = job();
        // Reading the library spawns one process per tape. Doing that on the UI thread would
        // freeze the animations right when the tape lands.
        let loaded = fetch();
        let _ = weak.upgrade_in_event_loop(move |ui| {
            ui.set_busy(false);
            ui.set_output(output.text.trim().into());
            if output.ok {
                // A finished job closes the install page; for other jobs it is already closed.
                ui.set_adding(false);
            } else {
                ui.set_status(last_line(&output.text).into());
            }
            apply(&ui, loaded);
        });
    });
}

pub fn load_tapes(ui: &AppWindow) {
    apply(ui, fetch());
}

/// Everything the UI shows, read from tapes without touching the UI so it can run on any thread.
struct Loaded {
    tapes: Vec<(String, String)>,
    active: i32,
    details: tapes::Details,
}

fn fetch() -> Loaded {
    let list = tapes::list();
    let details: Vec<tapes::Details> = list.iter().map(|tape| tapes::details(&tape.name)).collect();
    let active = tapes::active(&list);

    Loaded {
        tapes: list
            .iter()
            .zip(&details)
            .map(|(tape, details)| (tape.name.clone(), source(&details.description).to_string()))
            .collect(),
        active: active.map_or(-1, |i| i as i32),
        details: active.map(|i| details[i].clone()).unwrap_or_default(),
    }
}

fn apply(ui: &AppWindow, loaded: Loaded) {
    let infos: Vec<TapeInfo> = loaded
        .tapes
        .into_iter()
        .map(|(name, source)| TapeInfo { name: name.into(), source: source.into() })
        .collect();

    sync_tapes(ui, infos);
    ui.set_active(loaded.active);
    ui.set_active_insert_hooks(strings(loaded.details.insert_hooks));
    ui.set_active_eject_hooks(strings(loaded.details.eject_hooks));
    ui.set_active_targets(strings(loaded.details.targets));
}

/// Updates the tape list in place. Replacing the whole model rebuilds every cassette in the grid,
/// which shows up as a hitch right when an insert or eject finishes.
fn sync_tapes(ui: &AppWindow, infos: Vec<TapeInfo>) {
    let model = ui.get_tapes();
    let Some(rows) = model.as_any().downcast_ref::<VecModel<TapeInfo>>() else {
        ui.set_tapes(ModelRc::from(Rc::new(VecModel::from(infos))));
        return;
    };

    for (i, info) in infos.iter().enumerate() {
        match rows.row_data(i) {
            Some(row) if row == *info => {}
            Some(_) => rows.set_row_data(i, info.clone()),
            None => rows.push(info.clone()),
        }
    }
    while rows.row_count() > infos.len() {
        rows.remove(rows.row_count() - 1);
    }
}

fn strings(items: Vec<String>) -> ModelRc<SharedString> {
    let items: Vec<SharedString> = items.into_iter().map(SharedString::from).collect();
    ModelRc::from(Rc::new(VecModel::from(items)))
}

fn last_line(text: &str) -> &str {
    text.lines()
        .map(str::trim)
        .rfind(|line| !line.is_empty())
        .unwrap_or("something went wrong")
}

fn source(description: &str) -> &str {
    description
        .strip_prefix("Tape installed from ")
        .unwrap_or(description)
        .trim_start_matches("https://")
}

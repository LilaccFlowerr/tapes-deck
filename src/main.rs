slint::include_modules!();

mod tapes;
mod ui;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_add_tape(|| {
        println!("add a tape");
    });

    ui.run()
}

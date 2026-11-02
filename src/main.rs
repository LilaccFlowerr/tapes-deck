slint::include_modules!();

mod tapes;
mod ui;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui::setup(&ui);
    ui.run()
}

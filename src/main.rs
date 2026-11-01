slint::include_modules!();

mod tapes;
mod ui;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui::load_tapes(&ui);
    ui.run()
}

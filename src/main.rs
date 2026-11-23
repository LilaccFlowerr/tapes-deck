slint::include_modules!();

mod tapes;
mod ui;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    dither_slint::install!(ui);
    ui::setup(&ui);
    ui.run()
}

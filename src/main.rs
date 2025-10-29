#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod app;

fn main() -> eframe::Result {
  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

  let native_options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([215.0, 80.0])
      .with_min_inner_size([135.0, 75.0])
      .with_max_inner_size([300.0, 100.0])
      .with_always_on_top(),
    ..Default::default()
  };
  eframe::run_native(
    "I got bored and made an autoclicker",
    native_options,
    Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
  )
}

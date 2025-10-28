#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result {
  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

  let native_options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([290.0, 150.0])
      .with_min_inner_size([240.0, 100.0]),
    ..Default::default()
  };
  eframe::run_native(
    "I got bored and made an autoclicker",
    native_options,
    Box::new(|_| Ok(Box::new(eklik::App::new()))),
  )
}

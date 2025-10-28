use std::{
  sync::{
    Arc,
    atomic::{AtomicBool, AtomicU32, Ordering},
    mpsc,
  },
  thread,
  time::{Duration, Instant},
};
use enigo::{Button::Left, Direction::Click, Enigo, Mouse, Settings};

// variables for the app
pub struct App {
  speed: Arc<AtomicU32>,
  enabled: Arc<AtomicBool>,
  quit_tx: mpsc::Sender<()>,
}

// first frame of the app (basically an inital template)
impl App {
  pub fn new() -> Self {
    let enabled = Arc::new(AtomicBool::new(false));
    let speed = Arc::new(AtomicU32::new(100));

    let (quit_tx, quit_rx) = mpsc::channel();
    {
      let enabled = enabled.clone();
      let speed = speed.clone();

      let mut enigo = Enigo::new(&Settings::default()).unwrap();

      thread::spawn(move || {
        loop {
          if quit_rx.try_recv().is_ok() { break; }


          if enabled.load(Ordering::Relaxed) {
            let start = Instant::now();
            let period = Duration::from_secs_f64(1.0/speed.load(Ordering::Relaxed) as f64);

            let _ = enigo.button(Left, Click);

            let elapsed = start.elapsed();
            let dt = period.saturating_sub(elapsed);
            if !dt.is_zero() {
              thread::sleep(dt);
            }
          }
        }
      });
    }
    Self { enabled, speed, quit_tx }
  }
}

impl Drop for App {
  fn drop(&mut self) {let _ = self.quit_tx.send(()); }
}

// main app loop
impl eframe::App for App {
  // fn save(&mut self, storage: &mut dyn eframe::Storage) {
  //   eframe::set_value(storage, eframe::APP_KEY, self);
  // }

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // unpack vars
    let mut speed = self.speed.load(Ordering::Relaxed);
    let speed0 = speed;

    let mut enabled = self.enabled.load(Ordering::Relaxed);
    let enabled0 = enabled;

    // menubar
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
          if ui.button("Quit").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
          }
        });

        ui.add_space(16.0);

        egui::widgets::global_theme_preference_buttons(ui);
      });
    });

    // main page content
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.label("Speed:");
        ui.add(egui::DragValue::new(&mut speed).speed(1).range(1..=1000));
        ui.label("CPS");
      });

      enabled ^= ui
        .add(egui::Button::new(if enabled { "stop" } else { "start---" }).selected(enabled))
        .clicked();
    });

    // powered by egui statement
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
      powered_by_egui_and_eframe(ui);
    });

    // package vars if changed
    if speed != speed0 {
      self.speed.store(speed, Ordering::Relaxed);
    }
    if enabled != enabled0 {
      self.enabled.store(enabled, Ordering::Relaxed);
    }
  }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
  ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 0.0;
    ui.label("Powered by ");
    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
    ui.label(" and ");
    ui.hyperlink_to(
      "eframe",
      "https://github.com/emilk/egui/tree/master/crates/eframe",
    );
    ui.label(".");
  });
}
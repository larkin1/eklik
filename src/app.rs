use std::{
  sync::{
    Arc,
    atomic::{AtomicBool, AtomicU32, Ordering},
  },
  thread,
  time::{Duration, Instant},
};

use enigo::{Button::Left, Direction::Click, Enigo, Mouse, Settings};
use global_hotkey::{
  GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState::Pressed, hotkey::{Code, HotKey}
};

// variables for the app
pub struct App {
  speed: Arc<AtomicU32>,
  enabled: Arc<AtomicBool>,
  _hk_manager: global_hotkey::GlobalHotKeyManager,
}

// first frame of the app (basically an inital template)
impl App {
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    let ctx = cc.egui_ctx.clone();
    let enabled = Arc::new(AtomicBool::new(false));
    let speed = Arc::new(AtomicU32::new(100));

    let hk_manager = GlobalHotKeyManager::new().unwrap();
    let hk = HotKey::new(None, Code::F6);
    hk_manager.register(hk).unwrap();
    let hk_rx = GlobalHotKeyEvent::receiver().clone();

    {
      let enabled = enabled.clone();
      let speed = speed.clone();
      let ctx = ctx.clone();

      let mut enigo = Enigo::new(&Settings::default()).unwrap();

      thread::spawn(move || {
        let mut last_speed = 100u32;
        let mut period = Duration::new(0, 10000000);

        loop {
          let start = Instant::now();

          for ev in hk_rx.try_iter() {
            if ev.id == hk.id && ev.state == Pressed {
              enabled.fetch_xor(true, std::sync::atomic::Ordering::Relaxed);
              ctx.request_repaint();
            }
          }

          let enabled0 = enabled.load(Ordering::Relaxed);
          if enabled0 {
            let _ = enigo.button(Left, Click);
            let elapsed = start.elapsed();
            let dt = period.saturating_sub(elapsed);
            thread::sleep(dt);
          } else {
            let speed = speed.load(Ordering::Relaxed);
            if speed != last_speed {
              last_speed = speed;
              period = Duration::from_secs_f64(if speed >= 1000 {0.0} else {1.0 / speed as f64});
            }
            thread::sleep(Duration::from_millis(50));
          }
        }
      });
    }

    Self {
      enabled,
      speed,
      _hk_manager: hk_manager,
    }
  }
}

// main app loop
impl eframe::App for App {

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // unpack vars
    let mut speed = self.speed.load(Ordering::Relaxed);
    let speed0 = speed;

    let mut enabled = self.enabled.load(Ordering::Relaxed);
    let enabled0 = enabled;

    // main page content
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.add_enabled_ui(!enabled, |ui| {
      ui.horizontal(|ui| {
        ui.label("Speed:");
        ui.add(egui::DragValue::new(&mut speed).speed(1).range(1..=1000));
        ui.label("CPS");
      });

      enabled ^= ui
        .add(
          egui::Button::new(if enabled { "Stop (F6)" } else { "Start (F6)" }).selected(enabled),
        )
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
  });}
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
  ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing.x = 0.0;
    ui.hyperlink_to("Eklik ", "https://github.com/larkin1/eklik");
    ui.label("powered by ");
    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
    ui.label(" and ");
    ui.hyperlink_to(
      "eframe",
      "https://github.com/emilk/egui/tree/master/crates/eframe",
    );
    ui.label(".");
  });
}

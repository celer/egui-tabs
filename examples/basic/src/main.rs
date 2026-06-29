use eframe::egui;
use egui::Layout;
use egui_tabs::Tabs;
use std::cmp::Ordering;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tab Demo",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|_cc| Box::<MyApp>::default()),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Tabs::new(3)
            .height(32.0)
            .selected(1)
            .layout(Layout::left_to_right(egui::Align::Center))
            .show(ui, |ui, state| {
                let ind = state.index();
                let txt = if ind == 0 {
                    "Home"
                } else if ind == 1 {
                    "Settings"
                } else if ind == 2 {
                    "Profile"
                } else {
                    "Unknown"
                };

                let hovered = if state.is_hovered() { "h" } else { "" };
                let selected = if state.is_selected() { "s" } else { "" };

                let txt = if !hovered.is_empty() || !selected.is_empty() {
                    format!("{} ({}{})", txt, hovered, selected)
                } else {
                    txt.into()
                };

                let txt = if let Some(tab) = state.hovered_tab() {
                    match tab.cmp(&ind) {
                        Ordering::Equal => txt,
                        Ordering::Greater => format!("{} ->", txt),
                        Ordering::Less => format!("<- {}", txt),
                    }
                } else {
                    txt
                };

                ui.add(egui::Label::new(txt).selectable(false));
            });
    }
}

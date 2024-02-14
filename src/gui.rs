use eframe::egui;

use crate::solver::Solver;

pub fn initialize_egui<'a>(solver: &impl Solver<'a>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Word Ladder Solver",
        options,
        Box::new(|cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    origin: String,
    target: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            target: Default::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Word Ladder Solver");
            ui.horizontal(|ui| {
                let origin_label = ui.label("Origin: ");
                let mut origin = ui
                    .text_edit_singleline(&mut self.origin)
                    .labelled_by(origin_label.id);

                /* if origin.changed() {
                    self.origin.make_ascii_uppercase();
                    origin.mark_changed();
                } */
            });

            ui.horizontal(|ui| {
                let target_label = ui.label("Target: ");
                let mut target = ui
                    .text_edit_singleline(&mut self.target)
                    .labelled_by(target_label.id);
                /*  if target.changed() {
                    self.target.make_ascii_uppercase();
                    target.mark_changed();
                } */
            });
        });
    }
}

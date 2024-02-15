use anyhow::{anyhow, Result};
use eframe::{egui, AppCreator};
use egui::Color32;

use crate::solver::Solver;

pub fn initialize_egui<'a, S: Solver<'a>>(solver: &'a S) -> Result<()> {
    let options = eframe::NativeOptions::default();

    let ac = AppCreator::from(Box::new(|_cc| Box::new(MyApp::new(solver))));

    eframe::run_native("Word Ladder Solver", options, ac)
        .map_err(|err| anyhow!("eframe Error: {err:?}"))
}

struct MyApp<'a, S: Solver<'a>> {
    solver: &'a S,
    origin: String,
    target: String,
}

impl<'a, S: Solver<'a>> MyApp<'a, S> {
    fn new(solver: &'a S) -> Self {
        Self {
            solver,
            origin: Default::default(),
            target: Default::default(),
        }
    }
}

impl<'a, S: Solver<'a>> eframe::App for MyApp<'a, S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Word Ladder Solver");

            egui::Grid::new("words")
                .num_columns(3)
                .spacing([10.0, 4.0])
                .max_col_width(150.0)
                .striped(false)
                .show(ui, |ui| {
                    let origin_label = ui.label("Origin: ");
                    let mut origin = ui
                        .text_edit_singleline(&mut self.origin)
                        .labelled_by(origin_label.id);

                    if origin.changed() {
                        self.origin.make_ascii_uppercase();
                        origin.mark_changed();
                    }

                    let bytes = self.origin.as_bytes();

                    if !self.solver.word_exists(bytes) {
                        ui.colored_label(Color32::RED, "Not found");
                    }

                    ui.end_row();

                    let target_label = ui.label("Target: ");
                    let mut target = ui
                        .text_edit_singleline(&mut self.target)
                        .labelled_by(target_label.id);

                    if target.changed() {
                        self.target.make_ascii_uppercase();
                        target.mark_changed();
                    }

                    ui.colored_label(Color32::RED, "Not found");

                    ui.end_row();
                });
        });
    }
}

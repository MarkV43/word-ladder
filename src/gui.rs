use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use eframe::{egui, AppCreator};
use egui::Color32;

use crate::solver::Solver;

pub fn initialize_egui<S: Solver + 'static>(solver: S) -> Result<()> {
    let options = eframe::NativeOptions::default();

    let ac = AppCreator::from(Box::new(|_cc| Box::new(MyApp::new(solver))));

    eframe::run_native("Word Ladder Solver", options, ac)
        .map_err(|err| anyhow!("eframe Error: {err:?}"))
}

struct MyApp<S: Solver> {
    solver: S,
    origin: String,
    target: String,
    solution: Option<Result<Vec<String>, String>>,
    duration: Option<Duration>,
}

impl<S: Solver> MyApp<S> {
    fn new(solver: S) -> Self {
        Self {
            solver,
            origin: Default::default(),
            target: Default::default(),
            solution: None,
            duration: None,
        }
    }
}

impl<S: Solver> eframe::App for MyApp<S> {
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
                        self.solution = None;
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
                        self.solution = None;
                    }

                    let bytes = self.target.as_bytes();

                    if !self.solver.word_exists(bytes) {
                        ui.colored_label(Color32::RED, "Not found");
                    }

                    ui.end_row();
                });

            if ui.button("Solve").clicked() {
                let origin = self.origin.as_bytes();
                let target = self.target.as_bytes();

                let t0 = Instant::now();

                self.solution = Some(
                    self.solver
                        .solve(origin, target)
                        .map_err(|_| "No solution found".to_owned()),
                );

                self.duration = t0.elapsed().into();
            }

            match &self.solution {
                Some(Ok(solution)) => {
                    ui.vertical(|ui| {
                        for word in solution {
                            ui.label(word);
                        }
                    });

                    ui.colored_label(Color32::LIGHT_BLUE, format!("{:?}", self.duration.unwrap()));
                }
                Some(Err(err)) => {
                    ui.colored_label(Color32::RED, err);
                }
                None => {}
            }
        });
    }
}

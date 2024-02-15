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
    exceptions: Vec<&'static [u8]>,
}

impl<S: Solver> MyApp<S> {
    fn new(solver: S) -> Self {
        Self {
            solver,
            origin: Default::default(),
            target: Default::default(),
            solution: None,
            duration: None,
            exceptions: Vec::new(),
        }
    }
}

impl<S: Solver> eframe::App for MyApp<S> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Word Ladder Solver");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
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
                        self.update_solution();
                    }

                    let mut to_be_updated = false;

                    match &self.solution {
                        Some(Ok(solution)) => {
                            ui.vertical(|ui| {
                                for word in solution {
                                    ui.horizontal(|ui| {
                                        ui.label(word);

                                        if ui.button("×").clicked() {
                                            let bytes = word.as_bytes();
                                            let word: &'static [u8] = self
                                                .solver
                                                .get_dictionary()
                                                .iter()
                                                .find(|&&w| w == bytes)
                                                .unwrap();

                                            self.exceptions.push(word);
                                            self.solver.set_exceptions(&self.exceptions);

                                            to_be_updated = true;
                                        }
                                    });
                                }
                            });

                            ui.colored_label(
                                Color32::LIGHT_BLUE,
                                format!("{:?}", self.duration.unwrap()),
                            );
                        }
                        Some(Err(err)) => {
                            ui.colored_label(Color32::RED, err);
                        }
                        None => {}
                    }

                    if to_be_updated {
                        self.update_solution();
                    }
                });

                ui.vertical(|ui| {
                    ui.heading("Exceptions:");

                    ui.spacing();

                    let mut to_be_removed = None;

                    for (ind, exc) in self.exceptions.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(String::from_utf8_lossy(exc));

                            if ui.button("×").clicked() {
                                to_be_removed = Some(ind);
                            }
                        });
                    }

                    if let Some(ind) = to_be_removed {
                        self.exceptions.remove(ind);
                        self.solver.set_exceptions(&self.exceptions);
                        self.update_solution();
                    }
                });
            });
        });
    }
}

impl<S: Solver> MyApp<S> {
    fn update_solution(&mut self) {
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
}

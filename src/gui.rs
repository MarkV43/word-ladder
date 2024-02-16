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
            origin: String::new(),
            target: String::new(),
            solution: None,
            duration: None,
            exceptions: Vec::new(),
        }
    }
}

impl<S: Solver> eframe::App for MyApp<S> {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Word Ladder Solver");

            let size = ui.available_size();

            ui.horizontal(|ui| {
                ui.set_max_size(size);
                ui.vertical(|ui| {
                    let mut size = size;
                    size.x /= 2.0;

                    ui.set_max_size(size);

                    let words = egui::Grid::new("words")
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
                                self.clear_solution();
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

                    ui.horizontal(|ui| {
                        if ui.button("Solve").clicked() {
                            self.update_solution();
                        }

                        if let Some(dur) = self.duration {
                            ui.colored_label(Color32::LIGHT_BLUE, format!("{:?}", dur));
                        }
                    });

                    ui.add_space(30.0);

                    let mut to_be_updated = false;

                    match &self.solution {
                        Some(Ok(solution)) => {
                            egui::ScrollArea::vertical()
                                .auto_shrink([true, true])
                                .show(ui, |ui| {
                                    egui::Grid::new("solution")
                                        .num_columns(2)
                                        .spacing([30.0, 4.0])
                                        .max_col_width(150.0)
                                        .striped(true)
                                        .show(ui, |ui| {
                                            for word in solution {
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

                                                ui.end_row();
                                            }
                                        });
                                });
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

                    ui.add_space(30.0);

                    let mut to_be_removed = None;

                    egui::Grid::new("exceptions")
                        .num_columns(2)
                        .spacing([30.0, 4.0])
                        .max_col_width(150.0)
                        .striped(true)
                        .show(ui, |ui| {
                            for (ind, exc) in self.exceptions.iter().enumerate() {
                                ui.label(String::from_utf8_lossy(exc));

                                if ui.button("×").clicked() {
                                    to_be_removed = Some(ind);
                                }

                                ui.end_row();
                            }
                        });

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

    fn clear_solution(&mut self) {
        self.solution = None;
        self.duration = None;
    }
}

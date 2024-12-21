use day06::{Result, activate_global_default_tracing_subscriber, process_part1};
use eframe::run_simple_native;
use egui::{CentralPanel, Label, Ui};
use indoc::indoc;
use tracing::{Level, event, instrument, span};

fn main() -> Result<()> {
        let _write_guard = activate_global_default_tracing_subscriber()?;
        let inp = indoc!["
                ....#.....
                ....^....#
                ..........
                ..#.......
                .......#..
                ..........
                .#........
                ........#.
                #.........
                ......#..."];

        let mut name = "Arthur".to_owned();
        let mut age = 42;
        let eframe_config = eframe::NativeOptions::default();
        let span = span!(Level::INFO, "starting eframe", name, age);
        eframe::run_simple_native("My egui App", eframe_config, move |ctx, _frame| {
                let _enter = span.enter();
                egui::CentralPanel::default().show(ctx, |ui| {
                        ui.add(Label::new("Hello World!"));
                        ui.label("A shorter and more convenient way to add a label.");
                        ui.heading("My egui Application");
                        ui.horizontal(|ui| {
                                let name_label = ui.label("Your name: ");
                                ui.text_edit_singleline(&mut name).labelled_by(name_label.id);
                                event![Level::TRACE, ?name, age, "hor"];
                        });
                        ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
                        if ui.button("Increment").clicked() {
                                age += 1;
                                println!("Button clicked!");
                                event![Level::DEBUG, ?age];
                        }
                        ui.label(format!("Hello '{name}', age {age}"));
                });
        })?;
        Ok(())
}

#[instrument(skip_all)]
fn ui_counter(ui: &mut Ui, counter: &mut i32) {
        // Put the buttons and label on the same row:
        ui.horizontal(|ui| {
                if ui.button("−").clicked() {
                        *counter -= 1;
                        event![Level::DEBUG, ?counter];
                }
                ui.label(counter.to_string());
                if ui.button("+").clicked() {
                        *counter += 1;
                        event![Level::DEBUG, ?counter];
                }
        });
}

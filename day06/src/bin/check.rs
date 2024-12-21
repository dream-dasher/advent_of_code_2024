use day06::{PopulatedMaze, Result, activate_global_default_tracing_subscriber, parse_input,
            support::error::ErrKindDay06};
use eframe::run_simple_native;
use egui::{CentralPanel, Label, Ui};
use indoc::indoc;
use tracing::{Level, event, instrument, span};

fn main() -> Result<()> {
        let _write_guard = activate_global_default_tracing_subscriber()?;
        let input = indoc!["
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

        let (maze, mb_guard) = parse_input(input)?;
        let guard = mb_guard.ok_or(ErrKindDay06::NoGuardFound {
                source_input: Some(input.to_string()),
        })?;
        let mut pop_maze = PopulatedMaze::new(maze, guard)?;

        let eframe_config = eframe::NativeOptions::default();
        const FIXED_STRING: &str = "------fixed string----";
        let mut my_string = String::from("my_string");
        run_simple_native("My egui App", eframe_config, move |ctx, _frame| {
                egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("ui.label(_)");
                        ui.add(Label::new("ui.add(Label::new(_))"));
                        // ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut my_string));
                        // let mut technically_mut = fixed_string;
                        let mut technically_mut = FIXED_STRING;
                        ui.add(egui::TextEdit::multiline(&mut technically_mut).desired_width(ui.available_width()));
                        let output = egui::TextEdit::singleline(&mut my_string).show(ui);
                        if let Some(text_cursor_range) = output.cursor_range {
                                use egui::TextBuffer as _;
                                let selected_chars = text_cursor_range.as_sorted_char_range();
                                let selected_text = my_string.char_range(selected_chars);
                                ui.label("Selected text: ");
                                ui.monospace(selected_text);
                        }
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

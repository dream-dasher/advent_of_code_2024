use day06::{PopulatedMaze, Result, activate_global_default_tracing_subscriber, parse_input,
            support::error::ErrKindDay06};
use eframe::run_simple_native;
use egui::{Key, Label, SidePanel, TopBottomPanel, Ui};
use indoc::indoc;
use tracing::{Level, event, instrument};

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
        let pop_maze = PopulatedMaze::new(maze.clone(), guard)?;
        let mut maze_state = MazeState::new(input);

        let mut input = input.to_string();
        let eframe_config = eframe::NativeOptions::default();
        let mut my_string = String::from("my_string");
        run_simple_native("Maze Code Expolorer", eframe_config, move |ctx, _frame| {
                SidePanel::left("left").show(ctx, |ui| {
                        event![Level::TRACE, "get_key"];
                        for key in [Key::ArrowLeft, Key::ArrowRight, Key::ArrowUp, Key::ArrowDown] {
                                // ¿TODO: how does debouncing work?  (I think we're just looking ad press && release)
                                if ui.input(|i| i.key_pressed(key)) {
                                        maze_state.move_cursor(key);
                                }
                        }

                        event![Level::TRACE, "display_maze"];

                        for (row_idx, line) in maze_state.maze_string.lines().enumerate() {
                                ui.horizontal(|ui| {
                                        for (col_idx, ch) in line.chars().enumerate() {
                                                if (row_idx, col_idx) == maze_state.cursor_pos {
                                                        ui.colored_label(egui::Color32::RED, ch.to_string())
                                                } else {
                                                        ui.label(ch.to_string())
                                                };
                                        }
                                });
                        }

                        event![Level::TRACE, "show cursor position, raw"];
                        ui.label(format!(
                                "Cursor position: ({}, {})",
                                maze_state.cursor_pos.0, maze_state.cursor_pos.1
                        ));

                        SidePanel::right("right").show(ctx, |ui| {
                                ui.label("ui.label(_)");
                                ui.add(Label::new("ui.add(Label::new(_))"));
                                ui.add(egui::TextEdit::multiline(&mut pop_maze.to_string())
                                        .code_editor()
                                        .desired_width(ui.available_width()));
                                ui.add_sized(
                                        ui.available_size(),
                                        egui::TextEdit::multiline(&mut input)
                                                .code_editor()
                                                .desired_width(ui.available_width()),
                                );
                        });

                        TopBottomPanel::bottom("bot").show(ctx, |ui| {
                                ui.add_sized(
                                        ui.available_size(),
                                        egui::TextEdit::multiline(&mut my_string).interactive(true),
                                );
                        })
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

struct MazeState {
        maze_string: String,
        cursor_pos:  (usize, usize), // (row, column)
        rows:        usize,
        cols:        usize,
}

impl MazeState {
        fn new(maze: &str) -> Self {
                let rows = maze.lines().count();
                let cols = maze.lines().next().unwrap_or("").len();
                Self {
                        maze_string: maze.to_string(),
                        cursor_pos: (0, 0),
                        rows,
                        cols,
                }
        }

        fn move_cursor(&mut self, key: Key) {
                match key {
                        Key::ArrowLeft => self.cursor_pos.1 = self.cursor_pos.1.saturating_sub(1),
                        Key::ArrowRight => {
                                if self.cursor_pos.1 < self.cols - 1 {
                                        self.cursor_pos.1 += 1;
                                }
                        }
                        Key::ArrowUp => self.cursor_pos.0 = self.cursor_pos.0.saturating_sub(1),
                        Key::ArrowDown => {
                                if self.cursor_pos.0 < self.rows - 1 {
                                        self.cursor_pos.0 += 1;
                                }
                        }
                        _ => {}
                }
        }
}

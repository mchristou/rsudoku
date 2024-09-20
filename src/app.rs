use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Padding, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::{
    io::{self},
    time::{Duration, Instant},
};

use crate::{puzzle::Puzzle, Difficulty};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct App {
    exit: bool,
    puzzle: Puzzle,
    selected_row: usize,
    selected_col: usize,
    timer: Instant,
    level: Difficulty,
    time_to_solve: Duration,
}

impl App {
    pub fn new(level: Difficulty) -> Self {
        App {
            exit: false,
            puzzle: Puzzle::new(level),
            selected_col: 0,
            selected_row: 0,
            timer: Instant::now(),
            level,
            time_to_solve: Duration::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(Duration::from_secs(1))?;
        }

        Ok(())
    }

    fn new_game(&mut self) {
        self.puzzle = Puzzle::new(self.level);
        self.timer = Instant::now();
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self, timeout: Duration) -> io::Result<()> {
        // Poll for an event with a timeout to avoid blocking
        if event::poll(timeout)? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('N') | KeyCode::Char('n') => {
                self.new_game();
            }

            KeyCode::Char('R') | KeyCode::Char('r') => {
                self.puzzle.reset();
            }
            KeyCode::Left => {
                self.selected_col = self.selected_col.saturating_sub(1);
            }
            KeyCode::Right => {
                self.selected_col = self.selected_col.saturating_add(1).min(8);
            }
            KeyCode::Up => {
                self.selected_row = self.selected_row.saturating_sub(1);
            }
            KeyCode::Down => {
                self.selected_row = self.selected_row.saturating_add(1).min(8);
            }
            KeyCode::Char(c) if c.is_numeric() => {
                let num = c as u8 - b'0';
                self.puzzle
                    .insert_number(self.selected_row, self.selected_col, num);

                if self.puzzle.is_solved() {
                    self.time_to_solve = self.timer.elapsed();
                }
            }
            KeyCode::Backspace | KeyCode::Delete => {
                self.puzzle.clear_cell(self.selected_row, self.selected_col);
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.puzzle.is_solved() {
            let instructions = Title::from(Line::from(vec![
                " Quit ".into(),
                "<Q> ".blue().bold(),
                " New Game ".into(),
                "<N>".blue().bold(),
            ]));

            let text = Text::from(vec![
                Line::from(vec!["🎉 Congratulations! You solved the puzzle! 🎉".into()]),
                Line::from(vec![format!(
                    "Solved in: {0}",
                    format_duration(self.time_to_solve)
                )
                .into()]),
            ])
            .green()
            .bold()
            .centered();

            Paragraph::new(text)
                .centered()
                .bold()
                .block(
                    Block::default()
                        .padding(Padding::new(0, 0, area.height / 2 - 1, 0))
                        .title(instructions)
                        .title_position(Position::Bottom),
                )
                .render(area, buf);
        } else {
            // the outer block
            let instructions = Title::from(Line::from(vec![
                " Quit ".into(),
                "<Q> ".blue().bold(),
                " Delete ".into(),
                "<DEL>".blue().bold(),
                " Reset ".into(),
                "<R>".blue().bold(),
                " New Game ".into(),
                "<N>".blue().bold(),
            ]));

            let title = Title::from(" Sudoku ".bold());
            let timer = Title::from(Line::from(vec![
                format_duration(self.timer.elapsed()).into()
            ]));
            let block = Block::bordered()
                .title(title.alignment(Alignment::Center))
                .title(timer.alignment(Alignment::Right).position(Position::Bottom))
                .title(
                    instructions
                        .alignment(Alignment::Left)
                        .position(Position::Bottom),
                )
                .border_set(border::THICK);

            // inner space of outer block
            let inner_area = block.inner(area);

            let cell_width = inner_area.width / 9;
            let cell_height = inner_area.height / 9;
            let cell_size = std::cmp::min(cell_width, cell_height);

            let grid_width = 9 * cell_size;
            let grid_height = 9 * cell_size;

            // offset to put grid in center of inner area
            let horizontal_offset = (inner_area.width - grid_width) / 2;
            let vertical_offset = (inner_area.height - grid_height) / 2;

            let centered_inner_area = Rect {
                x: inner_area.x + horizontal_offset,
                y: inner_area.y + vertical_offset,
                width: grid_width,
                height: grid_height,
            };

            for row in 0..9 {
                for col in 0..9 {
                    let x = centered_inner_area.x + col as u16 * cell_size;
                    let y = centered_inner_area.y + row as u16 * cell_size;

                    let is_major_row = row % 3 == 0;
                    let is_major_col = col % 3 == 0;

                    let top_left_corner = match (is_major_row, is_major_col) {
                        (true, true) => "╬", // Major row and column
                        //
                        (true, false) => "╦", // Major row, regular column
                        //
                        (false, true) => "╠", // Major column, regular row
                        //
                        (false, false) => "┼", // Regular intersection
                    };

                    if row < 9 && x + cell_size <= centered_inner_area.x + grid_width {
                        let h_line = if is_major_row { "═" } else { "─" };
                        for i in 0..cell_size {
                            buf.set_string(x + i, y, h_line, Style::default());
                        }
                    }

                    if col < 9 && y + cell_size <= centered_inner_area.y + grid_height {
                        let v_line = if is_major_col { "║" } else { "│" };
                        for i in 0..cell_size {
                            buf.set_string(x, y + i, v_line, Style::default());
                        }
                    }

                    buf.set_string(x, y, top_left_corner, Style::default());

                    // render the Sudoku values in the grid cells
                    let cell = self.puzzle.grid()[row][col];
                    let (symbol, style) = if cell.value() == 0 {
                        (" ".into(), Style::default()) // empty cell
                    } else if cell.is_clue() {
                        (
                            cell.value().to_string(),
                            Style::default().fg(ratatui::style::Color::Yellow).bold(),
                        )
                    } else {
                        let cell_style = if cell.posible_wrong() {
                            Style::default().fg(ratatui::style::Color::Red).bold()
                        } else {
                            Style::default().fg(ratatui::style::Color::Blue).bold()
                        };

                        (cell.value().to_string(), cell_style)
                    };

                    // highlight the selected cell
                    let is_selected = self.selected_row == row && self.selected_col == col;
                    let cell_style = if is_selected {
                        style.underlined()
                    } else {
                        style
                    };

                    // center the symbol in the cell
                    let x_offset = (cell_size) / 2;
                    let y_offset = (cell_size) / 2;
                    buf.set_stringn(x + x_offset, y + y_offset, &symbol, 1, cell_style);
                }
            }

            // draw the final bottom horizontal line
            let last_row_y = centered_inner_area.y + grid_height;
            for col in 0..9 {
                let x = centered_inner_area.x + col as u16 * cell_size;
                for i in 0..=cell_size {
                    let symbol = if col % 3 == 0 && (i == 0 || i == cell_size) {
                        "╬"
                    } else {
                        "═"
                    };
                    buf.set_string(x + i, last_row_y, symbol, Style::default());
                }
            }

            // draw the final right vertical line
            let last_col_x = centered_inner_area.x + grid_width;
            for row in 0..9 {
                let y = centered_inner_area.y + row as u16 * cell_size;
                for i in 0..=cell_size {
                    let symbol = if (row % 3 == 0 && i == 0) || (i == cell_size) {
                        "╬"
                    } else {
                        "║"
                    };
                    buf.set_string(last_col_x, y + i, symbol, Style::default());
                }
            }

            block.render(area, buf);
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

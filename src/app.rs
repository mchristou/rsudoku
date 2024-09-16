use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, Widget,
    },
    DefaultTerminal, Frame,
};
use std::{
    io::{self},
    time::Duration,
    usize,
};

#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,
    pub sudoku_grid: Vec<Vec<u8>>,
    pub selected_row: usize,
    pub selected_col: usize,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            self.handle_events(Duration::from_secs(1))?;
        }

        Ok(())
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
            KeyCode::Left => {
                if self.selected_col > 0 {
                    self.selected_col -= 1;
                }
            }
            KeyCode::Right => {
                if self.selected_col < 8 {
                    self.selected_col += 1;
                }
            }
            KeyCode::Up => {
                if self.selected_row > 0 {
                    self.selected_row -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_row < 8 {
                    self.selected_row += 1;
                }
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
        // the outer block
        let title = Title::from(" Sudoku ".bold());
        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
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
                let cell_value = self.sudoku_grid[row][col];
                let (symbol, style) = if cell_value == 0 {
                    (" ".into(), Style::default()) // Empty cell
                } else {
                    (
                        cell_value.to_string(),
                        Style::default().fg(ratatui::style::Color::Yellow).bold(),
                    )
                };

                // highlight the selected cell
                let is_selected = self.selected_row == row && self.selected_col == col;
                let cell_style = if is_selected {
                    style.bg(ratatui::style::Color::Blue)
                } else {
                    style
                };

                // center the symbol in the cell
                let x_offset = (cell_size - 1) / 2;
                let y_offset = (cell_size - 1) / 2;
                buf.set_stringn(
                    x + x_offset,
                    y + y_offset,
                    symbol.to_string(),
                    1,
                    cell_style,
                );
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

use std::io;

use sudoku::{App, Difficulty};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new(Difficulty::Easy).run(&mut terminal);
    ratatui::restore();
    app_result
}

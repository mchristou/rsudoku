use std::io;

use sudoku::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        sudoku_grid: vec![vec![1; 9]; 9],

        ..Default::default()
    };
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}

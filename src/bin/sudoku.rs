use argh::FromArgs;
use std::io;

use sudoku::{App, Difficulty};

#[derive(FromArgs, Debug)]
/// Cli to play Sudoku
struct Sudoku {
    /// difficulty (options: easy, medium, hard, expert)
    #[argh(positional)]
    difficulty: Difficulty,
}

fn main() -> io::Result<()> {
    let args: Sudoku = argh::from_env();

    let mut terminal = ratatui::init();
    let app_result = App::new(args.difficulty).run(&mut terminal);
    ratatui::restore();

    app_result
}

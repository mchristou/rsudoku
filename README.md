# Sudoku CLI Game

This is a Sudoku CLI game implemented in Rust using [Ratatui](https://ratatui.rs/)

## Controls

- **Arrow keys**: Move between cells.
- **Number keys (1-9)**: Insert numbers into the selected cell.
- **Backspace / Delete**: Clear the selected cell.
- **N / n**: Start a new game.
- **R / r**: Reset the puzzle.
- **Q / q**: Quit the game.
- **H / h**: Hint on the selected cell.

## Installation

### Prerequisites
- Rust

### Clone the repository

```
git clone https://github.com/mchristou/rsudoku.git
cd rsudoku
```

### Built the project

```
cargo build --release
```

### Run the game
```
./sudoku [level]
```

### Contributing

Feel free to submit issues or pull requests for improvements, bug fixes, or new features.

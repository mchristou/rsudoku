use rand::seq::SliceRandom;
use std::{collections::HashSet, usize};

const SIZE: usize = 9;
const SUBGRID_SIZE: usize = 3;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy = 30,
    Medium = 29,
    Hard = 27,
    Expert = 26,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    value: u8,
    is_clue: bool,
}

impl Cell {
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn is_clue(&self) -> bool {
        self.is_clue
    }
}

pub type Grid = [[Cell; SIZE]; SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Puzzle {
    grid: Grid,
    clues: usize, // number of clues to keep in the puzzle
    is_solved: bool,
}

impl Puzzle {
    pub fn new(difficulty: Difficulty) -> Self {
        let mut puzzle = Puzzle {
            grid: [[Cell {
                value: 0,
                is_clue: true,
            }; SIZE]; SIZE],
            clues: difficulty as usize,
            is_solved: false,
        };
        puzzle.generate_full_solution();
        puzzle.remove_numbers();
        puzzle
    }

    pub fn grid(&self) -> Grid {
        self.grid
    }

    pub fn is_solved(&self) -> bool {
        self.is_solved
    }

    pub(crate) fn insert_number(&mut self, row: usize, col: usize, num: u8) {
        if self.grid[row][col].is_clue {
            return;
        }

        if self.grid[row][col].value == 0 {
            self.grid[row][col].value = num;

            self.is_solved = self.check_if_solved();
        }
    }

    pub(crate) fn clear_cell(&mut self, row: usize, col: usize) {
        if self.grid[row][col].is_clue {
            return;
        }

        self.grid[row][col].value = 0;
        self.is_solved = false;
    }

    pub(crate) fn reset(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                if !cell.is_clue {
                    cell.value = 0;
                }
            }
        }
    }

    fn check_if_solved(&self) -> bool {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.grid[row][col].value == 0 {
                    return false;
                }
            }
        }

        self.validate() // ensure the Sudoku is valid
    }

    fn generate_full_solution(&mut self) {
        fill_grid(&mut self.grid);
    }

    // Remove numbers from the grid while leaving 'clues' numbers
    fn remove_numbers(&mut self) {
        let mut rng = rand::thread_rng();
        let mut positions: Vec<(usize, usize)> = (0..SIZE)
            .flat_map(|r| (0..SIZE).map(move |c| (r, c)))
            .collect();
        positions.shuffle(&mut rng);

        let cells_to_remove = SIZE * SIZE - self.clues;
        for &(row, col) in &positions[..cells_to_remove] {
            self.grid[row][col] = Cell {
                value: 0,
                is_clue: false,
            };
        }
    }

    // Validate if the current grid is a valid Sudoku solution
    pub fn validate(&self) -> bool {
        validate_sudoku(&self.grid)
    }
}

// recursive function to fill the grid with numbers that follow Sudoku rules
fn fill_grid(grid: &mut Grid) -> bool {
    let numbers: Vec<u8> = (1..=9).collect();
    for row in 0..SIZE {
        for col in 0..SIZE {
            if grid[row][col].value == 0 {
                let mut choices = numbers.clone();
                choices.shuffle(&mut rand::thread_rng());

                for num in choices {
                    if is_safe(grid, row, col, num) {
                        grid[row][col].value = num;
                        if fill_grid(grid) {
                            return true;
                        }
                        grid[row][col].value = 0;
                    }
                }

                return false; // Backtrack
            }
        }
    }
    true
}

// check if placing the number is safe in the current position
fn is_safe(grid: &Grid, row: usize, col: usize, num: u8) -> bool {
    !is_in_row(grid, row, num)
        && !is_in_col(grid, col, num)
        && !is_in_subgrid(
            grid,
            row - row % SUBGRID_SIZE,
            col - col % SUBGRID_SIZE,
            num,
        )
}

fn is_in_row(grid: &Grid, row: usize, num: u8) -> bool {
    grid[row].iter().any(|cell| cell.value == num)
}

fn is_in_col(grid: &Grid, col: usize, num: u8) -> bool {
    for row in 0..SIZE {
        if grid[row][col].value == num {
            return true;
        }
    }
    false
}

fn is_in_subgrid(grid: &Grid, start_row: usize, start_col: usize, num: u8) -> bool {
    for row in 0..SUBGRID_SIZE {
        for col in 0..SUBGRID_SIZE {
            if grid[row + start_row][col + start_col].value == num {
                return true;
            }
        }
    }
    false
}

// Validate the entire grid for a valid Sudoku solution
fn validate_sudoku(grid: &Grid) -> bool {
    for row in 0..SIZE {
        if !is_valid_set(&grid[row].iter().map(|cell| cell.value).collect::<Vec<_>>()) {
            return false;
        }
    }

    for col in 0..SIZE {
        let mut column: Vec<u8> = Vec::new();
        for row in 0..SIZE {
            column.push(grid[row][col].value);
        }
        if !is_valid_set(&column) {
            return false;
        }
    }

    for row in (0..SIZE).step_by(SUBGRID_SIZE) {
        for col in (0..SIZE).step_by(SUBGRID_SIZE) {
            let mut subgrid: Vec<u8> = Vec::new();
            for i in 0..SUBGRID_SIZE {
                for j in 0..SUBGRID_SIZE {
                    subgrid.push(grid[row + i][col + j].value);
                }
            }
            if !is_valid_set(&subgrid) {
                return false;
            }
        }
    }

    true
}

fn is_valid_set(nums: &[u8]) -> bool {
    let mut set = HashSet::new();
    for &num in nums {
        if num != 0 && !set.insert(num) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EASY_CLUES: usize = 29;
    const MEDIUM_CLUES: usize = 28;
    const HARD_CLUES: usize = 27;
    const EXPERT_CLUES: usize = 26;

    #[test]
    fn test_puzzle_generation_easy() {
        let puzzle = Puzzle::new(Difficulty::Easy);
        assert_eq!(puzzle.clues, EASY_CLUES);
        let empty_cells = puzzle
            .grid()
            .iter()
            .flatten()
            .filter(|cell| cell.value == 0)
            .count();
        assert_eq!(empty_cells, SIZE * SIZE - EASY_CLUES);
    }

    #[test]
    fn test_puzzle_generation_medium() {
        let puzzle = Puzzle::new(Difficulty::Medium);
        assert_eq!(puzzle.clues, MEDIUM_CLUES);
        let empty_cells = puzzle
            .grid()
            .iter()
            .flatten()
            .filter(|cell| cell.value == 0)
            .count();
        assert_eq!(empty_cells, SIZE * SIZE - MEDIUM_CLUES);
    }

    #[test]
    fn test_puzzle_generation_hard() {
        let puzzle = Puzzle::new(Difficulty::Hard);
        assert_eq!(puzzle.clues, HARD_CLUES);
        let empty_cells = puzzle
            .grid()
            .iter()
            .flatten()
            .filter(|cell| cell.value == 0)
            .count();
        assert_eq!(empty_cells, SIZE * SIZE - HARD_CLUES);
    }

    #[test]
    fn test_puzzle_generation_expert() {
        let puzzle = Puzzle::new(Difficulty::Expert);
        assert_eq!(puzzle.clues, EXPERT_CLUES);
        let empty_cells = puzzle
            .grid()
            .iter()
            .flatten()
            .filter(|cell| cell.value == 0)
            .count();
        assert_eq!(empty_cells, SIZE * SIZE - EXPERT_CLUES);
    }

    #[test]
    fn test_grid_has_valid_solution_after_generation() {
        let puzzle = Puzzle::new(Difficulty::Medium);
        assert!(puzzle.validate());
    }

    #[test]
    fn test_is_valid_set() {
        let valid_row = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert!(is_valid_set(&valid_row));

        let invalid_row = vec![1, 2, 3, 4, 5, 5, 7, 8, 9];
        assert!(!is_valid_set(&invalid_row));
    }

    #[test]
    fn test_is_safe() {
        let mut grid = [[Cell {
            value: 0,
            is_clue: false,
        }; SIZE]; SIZE];
        grid[0][0] = Cell {
            value: 1,
            is_clue: true,
        };
        grid[0][1] = Cell {
            value: 2,
            is_clue: true,
        };
        grid[0][2] = Cell {
            value: 3,
            is_clue: true,
        };

        assert!(!is_safe(&grid, 0, 3, 1)); // 1 already in the row
        assert!(is_safe(&grid, 1, 3, 4)); // 4 is safe to place
    }
}
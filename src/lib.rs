mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Representation of a cell
pub enum Cell {
    Alive = 1,
    Dead = 0,
}

#[wasm_bindgen]
/// Representation of a wrapping universe
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    /// Returns the width of the universe
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Sets the width of the universe and resets all cells to dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Returns the height of the universe
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Sets the height of the universe and resets all cells to dead state
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    /// Returns the cell contents as a pointer
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Returns the vector index of a cell row and column
    fn get_index(&self, row: u32, column: u32) -> usize {
        ((row * self.width) + column) as usize
    }

    /// Counts the number of live neighbours at a given cell
    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_col) % self.height;
                let neighbour_column = (column + delta_row) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_column);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    /// Applys Conway's four rules to advance the state of the universe
    pub fn tick(&mut self) {
        // Define next set of cells
        let mut next = self.cells.clone();

        // For each cell update according to rules
        for row in 0..self.height {
            for col in 0..self.width {
                // Determine cell index
                let idx = self.get_index(row, col);

                // Get immutable copy of cell and count live neighbours
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                // Determine next cell value
                let next_cell = match (cell, live_neighbours) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                // Update next cell
                next[idx] = next_cell;
            }
        }

        // Update universe
        self.cells = next;
    }

    /// Creates and returns a new universe
    pub fn new() -> Universe {
        utils::set_panic_hook();

        // Set width and height to 68
        let width = 64;
        let height = 64;

        // Set cells to an 'interesting' pattern
        let cells = (0..width * height)
            .map(|i| {
                // If cell is multiple of 2 or 7 it is alive
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        // Construct and return universe
        Universe {
            width,
            height,
            cells,
        }
    }

    /// Renders the universe as a string representation
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    /// Format implementation for the Display trait
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Universe {
    /// Get the dead or alive status for each cell in the universe
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive or dead by passing an array of row and column pairs
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

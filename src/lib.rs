use colored::Colorize;
use rand::Rng;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]

pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

// ndarray crate can be used for multidimensional vector
// or just use single vector here
// use usize

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let mut rng = rand::thread_rng();
        let capacity = width * height;
        let mut cells = Vec::new();

        for _ in 0..capacity {
            // 0 (inclusive) to 3 (inclusive)
            // about a quarter will be alive
            let cell_value: u32 = rng.gen_range(0..=3);
            match cell_value {
                1 => cells.push(Cell::Alive),
                _ => cells.push(Cell::Dead),
            };
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_counts(&self) -> Vec<u32> {
        let mut vec: Vec<u32> = Vec::new();
        let mut idx = 0;
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for _cell in line {
                let row = idx / self.width;
                let col = idx % self.width;
                let mut count = 0;

                // 1, 2, adjacent left and right
                if Cell::Alive
                    == self.cells[self.get_index((self.width + row - 1) % self.width, col)]
                {
                    count += 1
                }
                if Cell::Alive
                    == self.cells[self.get_index((self.width + row + 1) % self.width, col)]
                {
                    count += 1
                }
                // 3, 4, adjacent top and bottom
                if Cell::Alive
                    == self.cells[self.get_index(row, (self.width + col - 1) % self.width)]
                {
                    count += 1
                }
                if Cell::Alive
                    == self.cells[self.get_index(row, (self.width + col + 1) % self.width)]
                {
                    count += 1
                }
                // 5, 6, top left and top right
                if Cell::Alive
                    == self.cells[self.get_index(
                        (self.width + row - 1) % self.width,
                        (self.width + col - 1) % self.width,
                    )]
                {
                    count += 1
                }
                if Cell::Alive
                    == self.cells[self.get_index(
                        (self.width + row + 1) % self.width,
                        (self.width + col - 1) % self.width,
                    )]
                {
                    count += 1
                }
                // 7, 8, bottom left and bottom right
                if Cell::Alive
                    == self.cells[self.get_index(
                        (self.width + row - 1) % self.width,
                        (self.width + col + 1) % self.width,
                    )]
                {
                    count += 1
                }
                if Cell::Alive
                    == self.cells[self.get_index(
                        (self.width + row + 1) % self.width,
                        (self.width + col + 1) % self.width,
                    )]
                {
                    count += 1
                }

                vec.push(count);
                idx += 1;
            }
        }
        vec
    }

    pub fn evolve(&mut self) {
        let live_counts = self.live_neighbor_counts();
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let count = live_counts[idx];
                let cell = self.cells[idx];

                let next_cell: Cell = match (cell, count) {
                    // Any live cell with fewer than 2 live neighbors dies as if by underpopulation
                    (Cell::Alive, n) if n < 2 => Cell::Dead,
                    // Any live cell with more than 3 live neighbors dies as if by overpopulation
                    (Cell::Alive, n) if n > 3 => Cell::Dead,
                    // Any dead cell with three live neighbours becomes a live cell.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other live cells not yet eliminated survive.
                    // Similarly, all other dead cells stay dead.
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in row {
                if let Cell::Dead = cell {
                    write!(f, "{}", "○ ".black())?;
                } else {
                    write!(f, "{}", "● ".cyan())?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

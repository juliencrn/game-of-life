#![no_std]
// mod utils;

extern crate js_sys;
extern crate web_sys;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

mod utils;

// use web_sys::console;

// pub struct Timer<'a> {
//     name: &'a str,
// }

// impl<'a> Timer<'a> {
//     pub fn new(name: &'a str) -> Timer<'a> {
//         console::time_with_label(name);
//         Timer { name }
//     }
// }

// impl<'a> Drop for Timer<'a> {
//     fn drop(&mut self) {
//         console::time_end_with_label(self.name);
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

/// Methods not exposed to JavaScript.
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // Getting row, col using a modulo in a loop is costly
        // Instead we'll read the value in a if and lazy calc if is needed
        let north = if row == 0 { self.height - 1 } else { row - 1 };
        let south = if row == self.height - 1 { 0 } else { row + 1 };
        let west = if col == 0 { self.width - 1 } else { col - 1 };
        let east = if col == self.width - 1 { 0 } else { col + 1 };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, col);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, col);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    pub fn create_cells(size: usize) -> FixedBitSet {
        FixedBitSet::with_capacity(size * 3)
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    pub fn get_glider_pattern<'a>() -> &'a [(i32, i32)] {
        &[(-1, 0), (0, 1), (1, -1), (1, 0), (1, 1)]
    }

    pub fn get_pulsar_pattern<'a>() -> &'a [(i32, i32)] {
        &[
            // Shape 1
            (-6, -4),
            (-6, -3),
            (-6, -2),
            (-4, -6),
            (-3, -6),
            (-2, -6),
            (-4, -1),
            (-3, -1),
            (-2, -1),
            (-1, -4),
            (-1, -3),
            (-1, -2),
            // Shape 3
            (-6, 2),
            (-6, 3),
            (-6, 4),
            (-4, 1),
            (-3, 1),
            (-2, 1),
            (-4, 6),
            (-3, 6),
            (-2, 6),
            (-1, 2),
            (-1, 3),
            (-1, 4),
            // Shape 3
            (1, -4),
            (1, -3),
            (1, -2),
            (2, -6),
            (3, -6),
            (4, -6),
            (2, -1),
            (3, -1),
            (4, -1),
            (6, -4),
            (6, -3),
            (6, -2),
            // shape 4
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 1),
            (3, 1),
            (4, 1),
            (2, 6),
            (3, 6),
            (4, 6),
            (6, 2),
            (6, 3),
            (6, 4),
        ]
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;
        let size = (width * height) as usize;

        Universe {
            width,
            height,
            cells: Universe::create_cells(size),
        }
    }

    pub fn randomize(&mut self) {
        let size = (self.width * self.height) as usize;
        for i in 0..size {
            self.cells.set(i, js_sys::Math::random() < 0.5);
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = Universe::create_cells((width * self.height) as usize);
    }

    pub fn reset_cells(&mut self) {
        let size = (self.width * self.height) as usize;
        self.cells = Universe::create_cells(size);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = Universe::create_cells((self.width * height) as usize);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        // let _timer = Timer::new("\nUniverse::tick");
        let mut next = {
            // let _timer = Timer::new("Alloc next cells");
            self.cells.clone()
        };

        {
            // let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let new_value = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbors
                        // dies, as if caused by under-population.
                        (true, x) if x < 2 => false,
                        // Rule 2: Any live cell with two or three live neighbors
                        // lives on to the next generation.
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live
                        // neighbors dies, as if by overpopulation.
                        (true, x) if x > 3 => false,
                        // Rule 4: Any dead cell with exactly three live neighbors
                        // becomes a live cell, as if by reproduction.
                        (false, 3) => true,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };

                    // Skip update if value is the same
                    if cell == new_value {
                        continue;
                    }

                    next.set(idx, new_value);
                }
            }
        };

        // let _timer = Timer::new("free old cells");
        self.cells = next;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let mut next = self.cells.clone();
        next.set(
            idx,
            match cell {
                true => false,
                false => true,
            },
        );
        self.cells = next;
    }

    fn draw_pattern(&mut self, row: u32, col: u32, pattern: &[(i32, i32)]) {
        let mut next = self.cells.clone();

        for (r_row, r_col) in pattern {
            // Calc the real position from relative position + clicked cell
            let mut r: i32 = row as i32 + r_row;
            let mut c: i32 = col as i32 + r_col;

            // Out of window case
            if r < 0 {
                r = self.height() as i32 + r;
            }
            if c < 0 {
                c = self.width() as i32 + c;
            }

            let idx = self.get_index(r as u32, c as u32);
            let cell = self.cells[idx];
            next.set(
                idx,
                match cell {
                    true => false,
                    false => true,
                },
            );
        }

        self.cells = next;
    }

    pub fn draw_glider(&mut self, row: u32, col: u32) {
        let cells_to_draw = Universe::get_glider_pattern();
        self.draw_pattern(row, col, cells_to_draw);
    }

    pub fn draw_pulsar(&mut self, row: u32, col: u32) {
        let cells_to_draw = Universe::get_pulsar_pattern();
        self.draw_pattern(row, col, cells_to_draw);
    }
}

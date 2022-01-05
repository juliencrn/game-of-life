// mod utils;

extern crate js_sys;
extern crate web_sys;

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

mod utils;

// // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// // allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.height - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn create_cells(size: usize) -> FixedBitSet {
        FixedBitSet::with_capacity(size)
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

    pub fn get_glider_pattern() -> Vec<(i32, i32)> {
        vec![(-1, 0), (0, 1), (1, -1), (1, 0), (1, 1)]
    }

    pub fn get_pulsar_pattern() -> Vec<(i32, i32)> {
        let shape_1 = vec![
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
        ];
        let shape_2 = vec![
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
        ];
        let shape_3 = vec![
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
        ];
        let shape_4 = vec![
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
        ];
        let mut cells_to_draw: Vec<(i32, i32)> = vec![];

        cells_to_draw.extend(shape_1);
        cells_to_draw.extend(shape_2);
        cells_to_draw.extend(shape_3);
        cells_to_draw.extend(shape_4);

        cells_to_draw
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
        let cells = Universe::create_cells(size);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn randomify(&mut self) {
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
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbors
                        // dies, as if caused by under-population.
                        (true, x) if x < 2 => {
                            // log!("cell[{}, {}] now dies", row, col);
                            false
                        }
                        // Rule 2: Any live cell with two or three live neighbors
                        // lives on to the next generation.
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live
                        // neighbors dies, as if by overpopulation.
                        (true, x) if x > 3 => {
                            // log!("cell[{}, {}] now dies", row, col);
                            false
                        }
                        // Rule 4: Any dead cell with exactly three live neighbors
                        // becomes a live cell, as if by reproduction.
                        (false, 3) => {
                            // log!("cell[{}, {}] now lives", row, col);
                            true
                        }
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

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

    fn draw_pattern(&mut self, row: u32, col: u32, pattern: Vec<(i32, i32)>) {
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

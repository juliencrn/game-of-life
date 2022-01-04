//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate game_of_life;
use game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_tick() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_tick() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_tick();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_tick();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[cfg(test)]
pub fn input_toggle() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(3, 1), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_toggle() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1, 2), (2, 3)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_toggle_cell() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_toggle();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_toggle();

    // Let's toggle some cells
    input_universe.toggle_cell(3, 1); // Toggle to dead
    input_universe.toggle_cell(3, 3); // Toggle to dead
    input_universe.toggle_cell(1, 2); // Toggle to alive
    input_universe.toggle_cell(2, 3); // Toggle to alive

    // then see if the cells in the `Universe`s are the same.
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[cfg(test)]
pub fn expected_empty() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe
}

#[wasm_bindgen_test]
pub fn test_reset_cells() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_toggle();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_empty();

    // Reset the universe
    input_universe.reset_cells();

    // then see if the cells in the `Universe`s are the same.
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

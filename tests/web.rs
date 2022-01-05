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

#[cfg(test)]
pub fn expected_glider() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_draw_glider() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = expected_empty();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_glider();

    // Reset the universe
    input_universe.draw_glider(2, 2);

    // then see if the cells in the `Universe`s are the same.
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[cfg(test)]
pub fn initial_17_universe() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(17);
    universe.set_height(17);
    universe
}

#[cfg(test)]
pub fn expected_17_pulsar() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(17);
    universe.set_height(17);
    let shape_1 = vec![
        (2, 4),
        (2, 5),
        (2, 6),
        (4, 2),
        (5, 2),
        (6, 2),
        (4, 7),
        (5, 7),
        (6, 7),
        (7, 4),
        (7, 5),
        (7, 6),
    ];
    let shape_2 = vec![
        (2, 10),
        (2, 11),
        (2, 12),
        (4, 9),
        (5, 9),
        (6, 9),
        (4, 14),
        (5, 14),
        (6, 14),
        (7, 10),
        (7, 11),
        (7, 12),
    ];
    let shape_3 = vec![
        (9, 4),
        (9, 5),
        (9, 6),
        (10, 2),
        (11, 2),
        (12, 2),
        (10, 7),
        (11, 7),
        (12, 7),
        (14, 4),
        (14, 5),
        (14, 6),
    ];
    let shape_4 = vec![
        (9, 10),
        (9, 11),
        (9, 12),
        (10, 9),
        (11, 9),
        (12, 9),
        (10, 14),
        (11, 14),
        (12, 14),
        (14, 10),
        (14, 11),
        (14, 12),
    ];

    let mut all_shapes = vec![];

    all_shapes.extend(shape_1);
    all_shapes.extend(shape_2);
    all_shapes.extend(shape_3);
    all_shapes.extend(shape_4);

    universe.set_cells(&all_shapes);
    universe
}

#[wasm_bindgen_test]
pub fn test_draw_pulsar() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = initial_17_universe();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_17_pulsar();

    // Reset the universe
    input_universe.draw_pulsar(8, 8);

    // then see if the cells in the `Universe`s are the same.
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

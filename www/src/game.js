import { memory } from "game-of-life/game_of_life_bg";
import { Universe } from "game-of-life";

class GameOfLife {
    constructor(width, height) {
        const universe = Universe.new();
        universe.set_width(width)
        universe.set_height(height)
        universe.randomize();

        this.universe = universe;
        this.reseated = false
        this.width = universe.width();
        this.height = universe.height();
    }

    tick() {
        if (this.reseated) {
            this.randomize()
        }
        this.universe.tick()
    }

    randomize() {
        this.universe.randomize()
        if (this.reseated) {
            this.reseated = false
        }
    }

    getCells() {
        const byteOffset = this.universe.cells()
        // Divided by 8 because 8 cells per byte
        const size = (this.width * this.height) / 8;
        const cells = new Uint8Array(memory.buffer, byteOffset, size);
        return cells;
    }

    reset() {
        this.universe.reset_cells();
        this.reseated = true
    }

    setReseated(newValue) {
        if (this.reseated !== newValue) {
            this.reseated = newValue;
        }
    }

    drawPulsar(row, col) {
        this.universe.draw_pulsar(row, col);
    }

    drawGlider(row, col) {
        this.universe.draw_glider(row, col);
    }

    toggleCell(row, col) {
        this.universe.toggle_cell(row, col);
    }
}

export default GameOfLife;
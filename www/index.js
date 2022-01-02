import { memory } from "game-of-life/game_of_life_bg";
import { Universe } from "game-of-life";

const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const CELL_SIZE = 10; // px

// Create Universe and Canvas
const universe = Universe.new();
const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext('2d');

// Initial settings
let isPlaying = true
// universe.set_width(100)
// universe.set_height(100)

universe.randomify()

// Get useful variables
const width = universe.width();
const height = universe.height();

// Set the canvas size
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    // Divided by 8 because 8 cells per byte
    const size = (width * height) / 8;
    const cells = new Uint8Array(memory.buffer, cellsPtr, size);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = bitIsSet(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

const renderLoop = () => {
    if (!isPlaying) {
        return
    }
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
}

window.addEventListener("keypress", event => {
    if (event.code === "Space") {
        isPlaying = !isPlaying
    }

    if (isPlaying) {
        requestAnimationFrame(renderLoop);
    }
})

requestAnimationFrame(renderLoop);

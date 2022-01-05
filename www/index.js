import { memory } from "game-of-life/game_of_life_bg";
import { Universe } from "game-of-life";

const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const CELL_SIZE = 10; // px

// Create Universe and Canvas
const universe = Universe.new();
const canvas = document.getElementById("game-of-life-canvas");
const playPauseButton = document.getElementById("play-pause");
const resetButton = document.getElementById("reset");
const randomButton = document.getElementById("random");
const fpsInput = document.getElementById("fps");
const ctx = canvas.getContext('2d');

let ctrlPressed = false
let shiftPressed = false

let fps = Number(fpsInput.value || 50)

let animationId = null;
let reseated = true;

let then = 0
let fpsInterval = 1000 / fps

universe.set_width(48)
universe.set_height(48)

const width = universe.width();
const height = universe.height();

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

const play = () => {
    if (reseated) {
        reseated = false
        universe.randomify()
    }
    playPauseButton.textContent = "Pause";
    fps = Number(fpsInput.value || 50)
    fpsInterval = 1000 / fps;
    then = Date.now();
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "Play";
    cancelAnimationFrame(animationId);
    animationId = null;
};

const isPaused = () => {
    return animationId === null;
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
    // calc elapsed time since last loop
    const now = Date.now();
    const elapsed = now - then;

    // if enough time has elapsed, draw the next frame
    if (elapsed > fpsInterval) {
        // Get ready for next frame by setting then=now, but also adjust for your
        // specified fpsInterval not being a multiple of RAF's interval (16.7ms)
        then = now - (elapsed % fpsInterval);

        // Own proper logic
        universe.tick();

        drawGrid();
        drawCells();
    }

    animationId = requestAnimationFrame(renderLoop);
}

const togglePlay = () => {
    if (isPaused()) {
        play()
    } else {
        pause()
    }
}

window.addEventListener("keydown", event => {
    switch (event.key) {
        case " ":
            togglePlay()
            break;
        // Control key
        case "Meta":
            ctrlPressed = true
            break;
        case "Shift":
            shiftPressed = true
            break;
        default:
            break;
    }
})

window.addEventListener("keyup", event => {
    ctrlPressed = false
    shiftPressed = false
})

playPauseButton.addEventListener('click', () => {
    togglePlay()
})

canvas.addEventListener("click", event => {
    if (reseated) {
        reseated = false
    }

    // Get the clicked cell position
    const boundingRect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    // Catch Ctrl+click, Ctrl+Shift+click
    if (ctrlPressed) {
        if (shiftPressed) {
            universe.draw_pulsar(row, col);
        } else {
            universe.draw_glider(row, col);
        }
    } else {
        universe.toggle_cell(row, col);
    }


    drawGrid();
    drawCells();
})

resetButton.addEventListener("click", event => {
    if (!isPaused()) {
        pause()
    }

    universe.reset_cells();
    reseated = true;

    drawGrid();
    drawCells();
})

randomButton.addEventListener("click", event => {
    if (!isPaused()) {
        pause()
    }

    universe.reset_cells();
    universe.randomify()
    reseated = true;

    drawGrid();
    drawCells();
})

fpsInput.addEventListener("change", event => {
    fps = Number(event.target.value || 50)

    // Refresh if it was playing
    if (!isPaused()) {
        pause()
        play()
    }
})

play()

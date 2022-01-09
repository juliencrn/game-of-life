import GameOfLife from "./game";
import Drawer from "./drawer";
import { CELL_SIZE, elements } from "./constants";
import { Ticker } from "./ticker";
import Fps from "./fps";

// TODO: remove global vars
let ctrlPressed = false
let shiftPressed = false

const gameOfLife = new GameOfLife(100, 100)
const ctx = elements.canvas.getContext('2d');
const drawer = new Drawer(
    ctx,
    gameOfLife.width,
    gameOfLife.height
)

// Set Canvas dimensions
elements.canvas.height = (CELL_SIZE + 1) * gameOfLife.height + 1;
elements.canvas.width = (CELL_SIZE + 1) * gameOfLife.width + 1;

function redraw() {
    drawer.drawGrid();
    drawer.drawCells(gameOfLife.getCells());
}

const timer = new Fps()

const ticker = new Ticker(() => {
    // for (let i = 0; i < 10; i++) {
    timer.render();
    gameOfLife.tick();
    redraw()
    // }
})

const play = () => {
    elements.playPauseButton.textContent = "Pause";
    ticker.startAnimation()
};

const pause = () => {
    elements.playPauseButton.textContent = "Play";
    ticker.stopAnimation()
};

const togglePlay = () => ticker.isPaused() ? play() : pause()

play()

// EVENTS

elements.playPauseButton.addEventListener('click', () => {
    togglePlay()
})

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

window.addEventListener("keyup", () => {
    ctrlPressed = false
    shiftPressed = false
})

elements.canvas.addEventListener("click", event => {
    gameOfLife.setReseated(false)

    // Get the clicked cell position
    const boundingRect = elements.canvas.getBoundingClientRect();
    const scaleX = elements.canvas.width / boundingRect.width;
    const scaleY = elements.canvas.height / boundingRect.height;
    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;
    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), gameOfLife.height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), gameOfLife.width - 1);

    // Catch Ctrl+click, Ctrl+Shift+click
    if (ctrlPressed) {
        if (shiftPressed) {
            gameOfLife.drawPulsar(row, col)
        } else {
            gameOfLife.drawGlider(row, col)
        }
    } else {
        gameOfLife.toggleCell(row, col);
    }

    redraw()
})

elements.resetButton.addEventListener("click", () => {
    pause()
    gameOfLife.reset()
    redraw()
})

elements.randomButton.addEventListener("click", () => {
    pause()
    gameOfLife.randomize()
    redraw()
})

elements.fpsInput.addEventListener("change", event => {
    const fps = Number(event.target.value || 60)
    ticker.setFps(fps)
    ticker.relaunchAnimation()
})

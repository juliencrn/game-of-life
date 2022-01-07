export class Ticker {
    constructor(cb) {
        this.cb = cb
        this.fps = 60
        this.animationId = null
        this.then = 0
        this.fpsInterval = 1000 / this.fps
    }

    setFps(value) {
        this.fps = Number(value)
        this.fpsInterval = 1000 / Number(value)
    }

    relaunchAnimation() {
        if (!this.isPaused()) {
            this.stopAnimation()
            this.startAnimation()
        }
    }

    startAnimation() {
        this.then = Date.now();
        this.renderLoop();
    }

    stopAnimation() {
        cancelAnimationFrame(this.animationId);
        this.animationId = null;
    }

    isPaused() {
        return this.animationId === null
    }

    renderLoop() {
        // calc elapsed time since last loop
        const now = Date.now();
        const elapsed = now - this.then;

        // if enough time has elapsed, draw the next frame
        if (elapsed > this.fpsInterval) {
            // Get ready for next frame by setting then=now, but also adjust for your
            // specified fpsInterval not being a multiple of RAF's interval (16.7ms)
            this.then = now - (elapsed % this.fpsInterval);

            // Own proper logic
            this.cb()
        }

        this.animationId = requestAnimationFrame(() => this.renderLoop());
    }
}

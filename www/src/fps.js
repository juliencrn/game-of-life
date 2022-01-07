export default class Fps {
    constructor() {
        this.latest = document.getElementById("fps-latest");
        this.avg = document.getElementById("fps-avg");
        this.min = document.getElementById("fps-min");
        this.max = document.getElementById("fps-max");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and avg of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let avg = sum / this.frames.length;

        // Render the statistics.
        this.latest.textContent = Math.round(fps).toString()
        this.avg.textContent = Math.round(avg).toString()
        this.min.textContent = Math.round(min).toString()
        this.max.textContent = Math.round(max).toString()
    }
};
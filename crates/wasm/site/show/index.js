import Renderer from './renderer.js';
import CanvasRecorder from './CanvasRecorder.js';

let visualizerWorker = null;

function terminateWorker() {
    visualizerWorker.terminated = true;
    visualizerWorker.terminate();
    visualizerWorker = null;
}

function reloadWorker() {
    if (visualizerWorker) terminateWorker();
    visualizerWorker = new Worker("./worker-visualizer.js", { name: "visualizer" });
}

let hash = location.hash.substring(1);
let params = {};
for (let part of hash.split('&')) {
    let [key,value] = part.split('=');
    params[key] = decodeURIComponent(value);
}
const {year, day, part, input} = params;

const canvas = document.querySelector("canvas");
const ctx = canvas.getContext('2d');

function visualize() {
    visualizerWorker.postMessage({ year, day, part, input });
    let myWorker = visualizerWorker;

    myWorker.onmessage = (message) => {
        const renderer = new Renderer(message, ctx);

        const recorder = params.download ? new CanvasRecorder(canvas) : null;
        if (recorder) recorder.start();

        function render(time) {
          if (myWorker.terminated) {
            console.log('[main] Aborting rendering from terminated');
          } else if (renderer.done) {
            console.log('[main] Rendering done');
            if (recorder) {
                recorder.stop();
                recorder.save(`advent-of-code-${year}-${day}-part${part}.webm`);
            }
            terminateWorker();
          } else {
              try {
                renderer.render();
                requestAnimationFrame(render);
              } catch (e) {
                console.error('Error when rendering', e);
                alert('Error when rendering: ' + e.message);
              }
          }
        }
        requestAnimationFrame(render);
    };
}

function goFullScreen() {
  if (document.fullscreenElement) {
    document.exitFullscreen();
  } else {
    document.documentElement.requestFullscreen();
  }
}

document.body.addEventListener('keyup', function (e) {
  if (e.keyCode == 13) {
    goFullScreen();
  }
});

//canvas.addEventListener('dblclick', goFullScreen);

new ResizeObserver(() => {
  canvas.width = canvas.clientWidth * window.devicePixelRatio;
  canvas.height = canvas.clientHeight * window.devicePixelRatio;
  reloadWorker();
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  visualize();
}).observe(canvas);


import Renderer from './renderer.js';
import CanvasRecorder from './CanvasRecorder.js';

let visualizerWorker = null;

function reloadWorker() {
    if (visualizerWorker) {
        visualizerWorker.terminated = true;
        visualizerWorker.terminate();
    }
    visualizerWorker = new Worker("./worker-visualizer.js", { name: "visualizer" });
}

let hash = location.hash.substring(1);
let params = {};
for (let part of hash.split('&')) {
    let [key,value] = part.split('=');
    params[key] = decodeURIComponent(value);
}
const {year, day, part, input} = params;

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext('2d');

function visualize() {
    visualizerWorker.postMessage({ year, day, part, input });
    let myWorker = visualizerWorker;

    myWorker.onmessage = (message) => {
        console.log('got message from worker');
        const renderer = new Renderer(message, ctx);
        function render(time) {
          if (myWorker.terminated) {
              console.log('aborting terminated worker');
          } else {
              try {
                renderer.render();
              } catch (e) {
                  console.log('ignoring terminated worker');
                  return;
              }
              requestAnimationFrame(render);
          }
        }
        requestAnimationFrame(render);
    };
}

function goFullScreen() {
  if (!document.fullscreenElement) {
      canvas.requestFullscreen();
  }
}

document.body.addEventListener('keyup', function (e) {
  if (e.keyCode == 13) {
    goFullScreen();
  }
});

canvas.addEventListener('dblclick', goFullScreen);

new ResizeObserver(() => {
  canvas.width = canvas.clientWidth;;
  canvas.height = canvas.clientHeight;
  reloadWorker();
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  visualize();
}).observe(canvas);


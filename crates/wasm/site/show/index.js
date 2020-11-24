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

window.reloadWithParameters = (parameters) => {
    Object.assign(params, parameters);
    let hashString = '';
    for (const [key, value] of Object.entries(params)) {
        if (hashString) hashString += '&';
        hashString += key + '=' + encodeURIComponent(value);
    }
    window.location.hash = hashString;
    window.location.reload();
}

const canvas = document.getElementById("mainCanvas");
const ctx = canvas.getContext('2d');

const layer1Canvas = document.getElementById("layer1");
const layer1Ctx = layer1Canvas.getContext('2d');
const composedCanvas = document.getElementById("composed");
const composedCtx = composedCanvas.getContext('2d');

if (params.aspectRatio) {
    window.aspectRatio = parseFloat(params.aspectRatio);
    for (let canvas of document.querySelectorAll('canvas')) {
        canvas.style.height = (100 / window.aspectRatio) + 'vw';
        canvas.style.maxWidth = (100 * window.aspectRatio) + 'vh';
    }
}

function visualize() {
    const {year, day, part, input} = params;
    visualizerWorker.postMessage({ year, day, part, input });
    let myWorker = visualizerWorker;

    myWorker.onmessage = (message) => {
        const renderer = new Renderer(message, [ctx, layer1Ctx]);

        const recorder = params.download ? new CanvasRecorder(canvas) : null;
        if (recorder) recorder.start();

        function render(time) {
          if (myWorker.terminated) {
            console.log('[main] Aborting rendering from terminated');
          } else if (renderer.done) {
            console.log('[main] Rendering done');
            if (recorder) {
                recorder.stopAndSave(`Advent-of-Code-${year}-Day-${day}-Part-${part}.webm`);
                window.location.hash = window.location.hash.replace('&download=true', '');
                console.log('reloading');
                window.location.reload();
            } else {
                terminateWorker();
            }
          } else {
              try {
                renderer.render();

                /*
                if (recorder) {
                    composedCtx.clearRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
                    composedCtx.drawImage(canvas, 0, 0);
                    composedCtx.drawImage(layer1Canvas, 0, 0);
                }
                */

                if (renderer.delay) {
                    setTimeout(render, renderer.delay);
                    renderer.delay = false;
                } else {
                    requestAnimationFrame(render);
                }
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
  } else if (e.key == 's') {
    window.reloadWithParameters({download:true});
  }
});

//canvas.addEventListener('dblclick', goFullScreen);
setTimeout(() => {
new ResizeObserver(() => {
  //const scaleFactor = params.download ? 1 : window.devicePixelRatio;
  const scaleFactor = window.devicePixelRatio;
  canvas.width = canvas.clientWidth * scaleFactor;
  canvas.height = canvas.clientHeight * scaleFactor;

  layer1Canvas.width = canvas.width;
  layer1Canvas.height = canvas.height;
  // TODO: Only have a compose canvas if recording.
  composedCanvas.width = canvas.width;
  composedCanvas.height = canvas.height;

  reloadWorker();
  visualize();
}).observe(canvas);
}, 0);


import Renderer from './renderer.js';
import CanvasRecorder from './CanvasRecorder.js';
import {createAudioPlayer} from './audio-player.js';

let visualizerWorker = null;

function terminateWorker() {
  console.log('Terminating earlier worker');
  visualizerWorker.terminated = true;
  visualizerWorker.terminate();
  visualizerWorker = null;
}

let hash = location.hash.substring(1);
let params = {};

for (let part of hash.split('&')) {
  let [key, value] = part.split('=');
  params[key] = decodeURIComponent(value);
}

function reloadWithParameters(parameters) {
  updateHash(parameters);
  window.location.reload();
}

function updateHash(parameters) {
  Object.assign(params, parameters);
  let hashString = '';
  for (const [key, value] of Object.entries(params)) {
    if (hashString) hashString += '&';
    hashString += key + '=' + encodeURIComponent(value);
  }
  window.location.hash = hashString;
}

const canvas = document.getElementById('mainCanvas');
const ctx = canvas.getContext('2d');

const layer1Canvas = document.getElementById('layer1');
const layer1Ctx = layer1Canvas.getContext('2d');
const composedCanvas = document.getElementById('composed');
const composedCtx = composedCanvas.getContext('2d');

if (params.aspectRatio) {
  onNewAspectRatio(parseFloat(params.aspectRatio));
}

function onNewAspectRatio(ratio) {
  if (window.aspectRatio == ratio) {
    return;
  }
  window.aspectRatio = ratio;
  updateHash({aspectRatio});
  for (let canvas of document.querySelectorAll('canvas')) {
    canvas.style.height = (100 / window.aspectRatio) + 'vw';
    canvas.style.maxWidth = (100 * window.aspectRatio) + 'vh';
  }
}

function visualize() {
  if (visualizerWorker) {
    terminateWorker();
  }
  visualizerWorker = new Worker('./worker-visualizer.js', {name: 'visualizer'});

  const {year, day, part, input} = params;
  visualizerWorker.postMessage({year, day, part, input});
  let myWorker = visualizerWorker;

  myWorker.onmessage = (message) => {
    if ('errorMessage' in message.data) {
        window.alert('Error from worker:\n' + message.data.errorMessage);
        window.location = '..';
        return;
    }

    const renderer = new Renderer(message, [ctx, layer1Ctx], onNewAspectRatio);
    window.renderer = renderer;

    const recorder = params.download ? new CanvasRecorder(composedCtx.canvas) : null;
    if (recorder) {
      recorder.start();
      document.getElementById('spinnerImage').src = 'recording.svg';
    } else {
      document.getElementById('spinner').style.visibility = 'hidden';
    }

    function render(time) {
      if (myWorker.terminated) {
        console.log('[main] Aborting rendering from terminated');
      } else if (renderer.done) {
        console.log('[main] Rendering done');
        if (recorder) {
          recorder.stopAndSave(generateFileName('webm'));
          updateHash({download: ''});
        }
        document.getElementById('spinner').style.visibility = 'visible';
        document.getElementById('spinnerImage').src = 'replay.svg';
        terminateWorker();
      } else {
        try {
          renderer.render();
          if (recorder) {
              composedCtx.fillStyle = 'rgb(13, 12, 26)';
              composedCtx.fillRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
              // composedCtx.clearRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
              composedCtx.drawImage(canvas, 0, 0);
              composedCtx.drawImage(layer1Canvas, 0, 0);
          }
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

    if (recorder) {
        let count = 0;
        function renderStartScreen() {
            count++;
            console.log('count', count);
            composedCtx.fillStyle = 'rgb(13, 12, 26)';
            composedCtx.fillRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
            composedCtx.textAlign = 'center';
            composedCtx.textBaseline = 'middle';
            composedCtx.fillStyle = 'white';
            const fontHeight = 80;
            composedCtx.font = fontHeight + 'px Monospace';
            composedCtx.fillText(`Advent of Code ${year}`, composedCtx.canvas.width/2, composedCtx.canvas.height/2 - fontHeight);
            composedCtx.fillText(`Day ${day} Part ${part}`, composedCtx.canvas.width/2, composedCtx.canvas.height/2 + fontHeight);
            if (count == 10) {
                requestAnimationFrame(render);
            } else {
                requestAnimationFrame(renderStartScreen);
            }
        }
        requestAnimationFrame(renderStartScreen);
    } else {
        requestAnimationFrame(render);
    }
  };
}

async function toggleFullScreen() {
  if (document.fullscreenElement) {
    document.exitFullscreen();
  } else {
    document.documentElement.requestFullscreen();
    if (window.aspectRatio && window.aspectRatio > 1.0) {
      await window.screen.orientation.lock('landscape-primary');
    }
  }
}

async function togglePause() {
  if (window.renderer.paused && !window.renderer.audioPlayer) {
    window.renderer.audioPlayer = await createAudioPlayer('./bounce.mp4');
  }
  window.renderer.paused = !window.renderer.paused;
}

function generateFileName(extension) {
  const {year, day, part, input} = params;
  return `Advent-of-Code-${year}-Day-${day}-Part-${part}.${extension}`;
}

function downloadImage() {
    composedCtx.fillStyle = 'rgb(13, 12, 26)';
    composedCtx.fillRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
    composedCtx.drawImage(canvas, 0, 0);
    composedCtx.drawImage(layer1Canvas, 0, 0);

    var url = composedCtx.canvas.toDataURL('image/png');
    const a = document.createElement('a');
    a.href = url;
    a.download = generateFileName('png');
    a.click();
}

document.body.addEventListener('keyup', async (e) => {
  switch (e.key) {
    case 'Escape':
      window.location = '..';
      break;
    case 'Enter':
      toggleFullScreen();
      break;
    case 'i': // Image.
      downloadImage();
      break;
    case 'p': // Pause.
    case ' ':
      await togglePause();
      break;
    case 'r': // Restart.
      visualize();
      break;
    case 'v': // Video.
      reloadWithParameters({download: true});
      break;
  }
});

document.documentElement.addEventListener('click', togglePause);
document.documentElement.addEventListener('dblclick', toggleFullScreen);

// https://web.dev/device-pixel-content-box/
function isDevicePixelContentBoxSupported() {
  return new Promise((resolve) => {
    const ro = new ResizeObserver((entries) => {
      resolve(entries.every((entry) => 'devicePixelContentBoxSize' in entry));
      ro.disconnect();
    });
    ro.observe(document.body, {box: ['device-pixel-content-box']});
  }).catch(() => false);
}

function saveContextState(ctx){
    const props = ['strokeStyle', 'fillStyle', 'globalAlpha', 'lineWidth',
    'lineCap', 'lineJoin', 'miterLimit', 'lineDashOffset', 'shadowOffsetX',
    'shadowOffsetY', 'shadowBlur', 'shadowColor', 'globalCompositeOperation',
    'font', 'textAlign', 'textBaseline', 'direction', 'imageSmoothingEnabled'];
    const state = {}
    for (let prop of props) state[prop] = ctx[prop];
    return state;
}

function restoreContextState(ctx, state){
    for (let prop in state) ctx[prop] = state[prop];
}

setTimeout(async () => {
  const devicePixelContentBoxSupported = await isDevicePixelContentBoxSupported();
  const observerOptions = devicePixelContentBoxSupported ? {box: ['device-pixel-content-box']} : {};

  let resizeCount = 0;

  new ResizeObserver((entries) => {
    resizeCount++;

    // TODO: For all layers, and reconstruct composed canvas?

    // Save a copy of the canvas:
    const tmpCanvas = document.createElement('canvas');
    tmpCanvas.width = canvas.width;
    tmpCanvas.height = canvas.height;
    tmpCanvas.getContext('2d').drawImage(canvas, 0, 0);

    let ctx = canvas.getContext('2d');

    // Resize canvas and restore context:
    const savedState = saveContextState(ctx);
    canvas.width = devicePixelContentBoxSupported ? entries[0].devicePixelContentBoxSize[0].inlineSize : (canvas.clientWidth * window.devicePixelRatio);
    canvas.height = devicePixelContentBoxSupported ? entries[0].devicePixelContentBoxSize[0].blockSize : (canvas.clientHeight * window.devicePixelRatio);
    restoreContextState(ctx, savedState);

    // Paint the old copy (scaled):
    ctx.setTransform(canvas.width/tmpCanvas.width, 0, 0, canvas.height/tmpCanvas.height, 0, 0);
    ctx.drawImage(tmpCanvas, 0, 0);

    // Setup the correct transform for future painting:
    ctx.setTransform(canvas.width, 0, 0, canvas.width, 0, 0);

    // TODO: Only have layer canvas if used, compose canvas if recording.
    layer1Canvas.width = canvas.width;
    layer1Canvas.height = canvas.height;
    composedCanvas.width = canvas.width;
    composedCanvas.height = canvas.height;

    if (resizeCount == 1) {
        setTimeout(visualize, 500);
    }
  }).observe(canvas, observerOptions);
}, 0);



import Renderer from './renderer.js';
import CanvasRecorder from './CanvasRecorder.js';

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
  console.log('updated params: ', params.download);
  window.location.hash = hashString;
}

const canvas = document.getElementById('mainCanvas');
const ctx = canvas.getContext('2d');

const layer1Canvas = document.getElementById('layer1');
const layer1Ctx = layer1Canvas.getContext('2d');
const composedCanvas = document.getElementById('composed');
// const composedCtx = composedCanvas.getContext('2d');

if (params.aspectRatio) {
  onNewAspectRatio(parseFloat(params.aspectRatio));
}

function onNewAspectRatio(ratio) {
  window.aspectRatio = ratio;
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

    const recorder = params.download ? new CanvasRecorder(canvas) : null;
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
          recorder.stopAndSave(`Advent-of-Code-${year}-Day-${day}-Part-${part}.webm`);
          updateHash({download: ''});
        }
        // document.getElementById('spinner').style.visibility = 'visible';
        // document.getElementById('spinnerImage').src = 'replay.svg';
        canvas.classList.remove('slide-in');
        canvas.classList.add('slide-out');
        terminateWorker();
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

async function toggleFullScreen() {
  if (document.fullscreenElement) {
    document.exitFullscreen();
  } else {
    document.documentElement.requestFullscreen();
    // if ('orientation' in window.screen)
    // TODO: Only lock orientation if non-square aspect ratio?

    if (window.aspectRatio && window.aspectRatio > 1.0) {
      await window.screen.orientation.lock('landscape-primary');
    }
  }
}

function togglePause() {
  window.renderer.paused = !window.renderer.paused;
}

document.body.addEventListener('keyup', (e) => {
  switch (e.key) {
    case 'Escape':
      window.location = '..';
      break;
    case 'Enter':
      toggleFullScreen();
      break;
    case 'p':
    case ' ':
      togglePause();
      break;
    case 'r':
      visualize();
      break;
    case 's':
      reloadWithParameters({download: true});
      break;
  }
});

document.body.addEventListener('dblclick', toggleFullScreen);

let resizeCount = 0;

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

setTimeout(async () => {
  const devicePixelContentBoxSupported = await isDevicePixelContentBoxSupported();
  console.log('devicePixelContentBoxSupported', devicePixelContentBoxSupported);
  const observerOptions = devicePixelContentBoxSupported ? {box: ['device-pixel-content-box']} : {};

  new ResizeObserver((entries) => {
    resizeCount++;

    // TODO: For other layer and composed canvas?
    const tmpCanvas = document.createElement('canvas');
    tmpCanvas.width = canvas.width;
    tmpCanvas.height = canvas.height;
    tmpCanvas.getContext('2d').drawImage(canvas, 0, 0);

    canvas.width = devicePixelContentBoxSupported ? entries[0].devicePixelContentBoxSize[0].inlineSize : (canvas.clientWidth * window.devicePixelRatio);
    canvas.height = devicePixelContentBoxSupported ? entries[0].devicePixelContentBoxSize[0].blockSize : (canvas.clientHeight * window.devicePixelRatio);

    // TODO: For other layer and composed canvas?
    let ctx = canvas.getContext('2d');
    ctx.setTransform(canvas.width/tmpCanvas.width, 0, 0, canvas.height/tmpCanvas.height, 0, 0);
    ctx.drawImage(tmpCanvas, 0, 0);
    ctx.setTransform(canvas.width, 0, 0, canvas.height, 0, 0);

    layer1Canvas.width = canvas.width;
    layer1Canvas.height = canvas.height;

    // TODO: Only have a compose canvas if recording.
    composedCanvas.width = canvas.width;
    composedCanvas.height = canvas.height;

    if (resizeCount == 1) visualize();
  }).observe(canvas, observerOptions);
}, 0);



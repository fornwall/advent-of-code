import Renderer from './renderer.js';
import CanvasRecorder from './CanvasRecorder.js';
import {createAudioPlayer} from './audio-player.js';

const visualizerWorker = new Worker('./worker-visualizer.js', {name: 'visualizer'});

const state = {
   phase: 'pageload',
   params: {},
};

let hash = location.hash.substring(1);
for (let part of hash.split('&')) {
  let [key, value] = part.split('=');
  state.params[key] = decodeURIComponent(value);
}

function reloadWithParameters(parameters) {
  updateHash(parameters);
  window.location.reload();
}

function updateHash(parameters) {
  Object.assign(state.params, parameters);
  let hashString = '';
  for (const [key, value] of Object.entries(state.params)) {
    if (hashString) hashString += '&';
    hashString += key + '=' + encodeURIComponent(value);
  }
  window.location.hash = hashString;
}

const canvas = document.getElementById('mainCanvas');
const ctx = canvas.getContext('2d');

const overlayCanvas = document.getElementById('layer1');
const overlayCtx = overlayCanvas.getContext('2d');

const composedCanvas = document.getElementById('composed');
const composedCtx = composedCanvas.getContext('2d');

if (state.params.aspectRatio) {
  onNewAspectRatio(parseFloat(state.params.aspectRatio));
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

function startVisualization() {
  const {year, day, part, input} = state.params;

  visualizerWorker.onmessage = (message) => {
    if ('errorMessage' in message.data) {
        window.alert('Error:\n\n' + message.data.errorMessage);
        window.location = '..';
        return;
    }

    const recorder = state.params.download ? new CanvasRecorder(composedCtx.canvas) : null;
    if (recorder) {
      recorder.start();
      document.getElementById('spinnerImage').src = 'recording.svg';
    } else {
      document.getElementById('spinner').style.visibility = 'hidden';
    }

    function startRendering() {
        const renderer = new Renderer(message, [ctx, overlayCtx], onNewAspectRatio, state.audioPlayer);
        window.renderer = renderer;

        function render(time) {
          if (renderer.done) {
            console.log('[main] Rendering done');
            if (recorder) {
              recorder.stopAndSave(generateFileName('webm'));
              updateHash({download: ''});
            }
            document.getElementById('spinner').style.visibility = 'visible';
            document.getElementById('spinnerImage').src = 'replay.svg';
          } else {
            try {
              renderer.render();
              if (recorder) {
                  composedCtx.fillStyle = 'rgb(13, 12, 26)';
                  composedCtx.fillRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
                  // composedCtx.clearRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
                  composedCtx.drawImage(canvas, 0, 0);
                  composedCtx.drawImage(overlayCanvas, 0, 0);
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

        requestAnimationFrame(render);
    }

    let count = 0;
    state.phase = 'onStartScreen';
    function renderStartScreen() {
            count++;
            const fontHeight = 80;
            const startScreenCanvases = recorder ? [ctx, composedCtx] : [ctx];
            const startRenderingNow = (recorder && count == 10) ||
              state.phase === 'startScreenClicked' ||
              localStorage.getItem('debug_autostart');

            for (const c of startScreenCanvases) {
                c.resetTransform();
                if (startRenderingNow) {
                    c.clearRect(0, 0, c.canvas.width, c.canvas.height);
                } else {
                    c.fillStyle = 'rgb(13, 12, 26)';
                    c.fillRect(0, 0, c.canvas.width, c.canvas.height);
                    c.textAlign = 'center';
                    c.textBaseline = 'middle';
                    c.fillStyle = 'white';
                    c.font = fontHeight + 'px Monospace';
                    c.fillText(`Advent of Code ${year}`, c.canvas.width/2, c.canvas.height/2 - fontHeight, c.canvas.width);
                    c.fillText(`Day ${day} Part ${part}`, c.canvas.width/2, c.canvas.height/2 + fontHeight, c.canvas.width);
                }
                if (c == ctx) c.setTransform(c.canvas.width, 0, 0, c.canvas.width, 0, 0);
            }

            if (startRenderingNow) {
                console.log('[main] Starting rendering...');
                state.phase = 'rendering';
                startRendering();
            } else {
                requestAnimationFrame(renderStartScreen);
            }
    }
    requestAnimationFrame(renderStartScreen);
  };

  visualizerWorker.postMessage({year, day, part, input});
}

async function toggleFullScreen() {
  if (document.fullscreenElement) {
    document.exitFullscreen();
  } else {
    document.documentElement.requestFullscreen();
    if (window.aspectRatio && window.aspectRatio > 1.0) {
      try {
        await window.screen.orientation.lock('landscape-primary');
      } catch (e) {}
    }
  }
}

async function togglePause() {
  if (state.phase === 'onStartScreen') {
      state.audioPlayer = await createAudioPlayer('./bounce.mp4');
      state.phase = 'startScreenClicked';
      return;
  }

  if (window.renderer.done) {
     reloadWithParameters({download: ''});
  }

  window.renderer.paused = !window.renderer.paused;
}

function generateFileName(extension) {
  const {year, day, part} = state.params;
  return `Advent-of-Code-${year}-Day-${day}-Part-${part}.${extension}`;
}

function downloadImage() {
    composedCtx.fillStyle = 'rgb(13, 12, 26)';
    composedCtx.fillRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
    composedCtx.drawImage(canvas, 0, 0);
    composedCtx.drawImage(overlayCanvas, 0, 0);

    const url = composedCtx.canvas.toDataURL('image/png');
    const a = document.createElement('a');
    a.href = url;
    a.download = generateFileName('png');
    a.click();
}

document.body.addEventListener('keyup', async(e) => {
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
      reloadWithParameters({download: ''});
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

function saveContextState(ctx) {
    const props = ['strokeStyle', 'fillStyle', 'globalAlpha', 'lineWidth',
    'lineCap', 'lineJoin', 'miterLimit', 'lineDashOffset', 'shadowOffsetX',
    'shadowOffsetY', 'shadowBlur', 'shadowColor', 'globalCompositeOperation',
    'font', 'textAlign', 'textBaseline', 'direction', 'imageSmoothingEnabled'];
    const state = {};
    for (let prop of props) state[prop] = ctx[prop];
    return state;
}

function restoreContextState(ctx, state) {
    for (const [key, value] of Object.entries(state)) {
        ctx[key] = value;
    }
}

setTimeout(async() => {
  const devicePixelContentBoxSupported = await isDevicePixelContentBoxSupported();
  const observerOptions = devicePixelContentBoxSupported ? {box: ['device-pixel-content-box']} : {};

  let resizeCount = 0;

  new ResizeObserver((entries) => {
    resizeCount++;

    // Save a copy of the canvas:
    // TODO: We should store the status text and re-render to avoid alpha problem and blurry text:
    for (const c of [overlayCtx, ctx]) {
      composedCanvas.width = canvas.width;
      composedCtx.clearRect(0, 0, composedCtx.canvas.width, composedCtx.canvas.height);
      composedCtx.drawImage(c.canvas, 0, 0);

      // Resize canvas and restore context:
      const savedState = saveContextState(c);
      const newWidth = devicePixelContentBoxSupported ?
            entries[0].devicePixelContentBoxSize[0].inlineSize :
            (canvas.clientWidth * window.devicePixelRatio);
      const newHeight = devicePixelContentBoxSupported ?
            entries[0].devicePixelContentBoxSize[0].blockSize :
            (canvas.clientHeight * window.devicePixelRatio);

      if (c.canvas.width == newWidth && c.canvas.height == newHeight) {
          console.log('Ignoring resize with same size');
          return;
      }

      c.canvas.width = newWidth;
      c.canvas.height = newHeight;
      restoreContextState(c, savedState);

      // Clear the old content, if any:
      c.resetTransform();
      c.clearRect(0, 0, c.canvas.width, c.canvas.height);
      // Paint the old copy (scaled):
      c.setTransform(c.canvas.width/composedCanvas.width, 0, 0, c.canvas.height/composedCanvas.height, 0, 0);
      c.drawImage(composedCanvas, 0, 0);

      // Setup the correct transform for future painting:
      if (c == ctx) {
          c.setTransform(canvas.width, 0, 0, canvas.width, 0, 0);
      } else {
          c.resetTransform();
      }
    }

    // TODO: Only have layer canvas if used, compose canvas if recording.
    composedCanvas.width = canvas.width;
    composedCanvas.height = canvas.height;

    if (resizeCount == 1) {
        startVisualization();
    }
  }).observe(canvas, observerOptions);
}, 0);

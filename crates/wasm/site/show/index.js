import Renderer from "./renderer.js";
import CanvasRecorder from "./CanvasRecorder.js";
import { AudioPlayer } from "./audio-player.js";

const visualizerWorker = new Worker("./worker-visualizer.js", {
  name: "visualizer",
});

const spinnerImage = document.getElementById("spinnerImage");

const PHASE_PAGE_LOAD = "pageload";
const PHASE_SHOWING_START_SCREEN = "showingstartscreen";
const PHASE_START_SCREEN_CLICKED = "startscreenclicked";

const state = {
  phase: PHASE_PAGE_LOAD,
  params: {},
  audioPlayer: new AudioPlayer("bounce.mp4", "pop.mp4"),
};

let hash = location.hash.substring(1);
for (let part of hash.split("&")) {
  let [key, value] = part.split("=");
  state.params[key] = decodeURIComponent(value);
}

function reloadWithParameters(parameters) {
  updateHash(parameters);
  window.location.reload();
}

function updateHash(parameters) {
  Object.assign(state.params, parameters);
  let hashString = "";
  for (const [key, value] of Object.entries(state.params)) {
    if (hashString) hashString += "&";
    hashString += key + "=" + encodeURIComponent(value);
  }
  window.location.hash = hashString;
}

const canvas = document.getElementById("mainCanvas");
const ctx = canvas.getContext("2d");

const overlayCanvas = document.getElementById("overlayCanvas");
const overlayCtx = overlayCanvas.getContext("2d");

const composedCanvas = document.getElementById("composed");
const composedCtx = composedCanvas.getContext("2d");

if (state.params.aspectRatio) {
  console.log("[main] Using aspect ratio from hash");
  onNewAspectRatio(parseFloat(state.params.aspectRatio));
}

function onNewAspectRatio(ratio) {
  if (state.aspectRatio == ratio) {
    return;
  }
  console.log("[main] New aspect ratio " + ratio);
  state.aspectRatio = ratio;
  for (let canvas of document.querySelectorAll("canvas")) {
    canvas.style.height = 100 / state.aspectRatio + "vw";
    canvas.style.maxWidth = 100 * state.aspectRatio + "vh";
  }
}

function sendMessageToWorker() {
  const { year, day, part, input } = state.params;
  visualizerWorker.postMessage({ year, day, part, input });
}

visualizerWorker.onmessage = (message) => {
  if ("errorMessage" in message.data) {
    window.alert(message.data.errorMessage);
    window.location = "..";
    return;
  } else if (message.data.done) {
    console.log("[main] Rust exited!");
    return;
  }

  if (state.params.download) {
    spinnerImage.src = "recording.svg";
  } else {
    document.getElementById("spinner").style.visibility = "hidden";
    spinnerImage.src = "replay.svg";
    spinnerImage.classList.remove("fade-out");
  }

  const rerun = !!window.renderer;
  const renderer = new Renderer(
    message,
    [ctx, overlayCtx],
    onNewAspectRatio,
    state.audioPlayer
  );
  window.renderer = renderer;

  function startRendering(renderer) {
    function render(time) {
      // TODO: Do not ignore time parameter.
      if (renderer.done) {
        console.log("[main] Rendering done");
        if (state.recorder) {
          setTimeout(() => {
            // Give time for last frames to be recorded:
            state.recorder.stopAndSave(generateFileName("webm"));
            updateHash({ download: "" });
          }, 1000);
        }
        document.getElementById("spinner").style.visibility = "visible";
        spinnerImage.classList.add("fade-out");
      } else {
        try {
          renderer.render();
          if (state.recorder) {
            composedCtx.fillStyle = "rgb(13, 12, 26)";
            composedCtx.fillRect(
              0,
              0,
              composedCtx.canvas.width,
              composedCtx.canvas.height
            );
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
          console.error("Error when rendering", e);
          alert("Error when rendering: " + e.message);
        }
      }
    }

    requestAnimationFrame(render);
  }

  let count = 0;
  state.phase = PHASE_SHOWING_START_SCREEN;
  function renderStartScreen() {
    if (state.phase == PHASE_START_SCREEN_CLICKED) {
      count++;
    }
    const fontHeight = 80;
    const startScreenCanvases = state.params.download
      ? [ctx, composedCtx]
      : [ctx];
    const startRenderingNow =
      (state.params.download && count == 10) ||
      (!state.params.download && state.phase === PHASE_START_SCREEN_CLICKED) ||
      rerun ||
      (localStorage.getItem("debug_autostart") && !state.params.download);

    for (const c of startScreenCanvases) {
      c.resetTransform();
      if (startRenderingNow) {
        c.clearRect(0, 0, c.canvas.width, c.canvas.height);
      } else {
        c.fillStyle = "rgb(13, 12, 26)";
        c.fillRect(0, 0, c.canvas.width, c.canvas.height);
        c.textAlign = "center";
        c.textBaseline = "middle";
        c.fillStyle = "white";
        c.font = fontHeight + "px Monospace";
        const { year, day, part } = state.params;
        c.fillText(
          `Advent of Code ${year}`,
          c.canvas.width / 2,
          c.canvas.height / 2 - fontHeight,
          c.canvas.width
        );
        c.fillText(
          `Day ${day} Part ${part}`,
          c.canvas.width / 2,
          c.canvas.height / 2 + fontHeight,
          c.canvas.width
        );
      }
      if (c == ctx) c.setTransform(c.canvas.width, 0, 0, c.canvas.width, 0, 0);
    }

    if (startRenderingNow) {
      state.phase = "rendering";
      startRendering(renderer);
    } else {
      requestAnimationFrame(renderStartScreen);
    }
  }
  requestAnimationFrame(renderStartScreen);
};

async function toggleFullScreen() {
  if (document.fullscreenElement) {
    document.exitFullscreen();
  } else {
    document.documentElement.requestFullscreen();
    if (state.aspectRatio && state.aspectRatio > 1.0) {
      try {
        await window.screen.orientation.lock("landscape-primary");
      } catch (e) {}
    }
  }
}

async function togglePause() {
  switch (state.phase) {
    case PHASE_PAGE_LOAD:
      break;
    case PHASE_SHOWING_START_SCREEN:
      const recordAudio = state.params.download && true;
      await state.audioPlayer.load(recordAudio);
      if (state.params.download) {
        state.recorder = recordAudio
          ? new CanvasRecorder(
              composedCtx.canvas,
              state.audioPlayer.createStream()
            )
          : new CanvasRecorder(composedCtx.canvas);
        state.recorder.start();
      }
      state.phase = PHASE_START_SCREEN_CLICKED;
      break;
    default:
      if (window.renderer.done) {
        sendMessageToWorker();
      } else {
        window.renderer.paused = !window.renderer.paused;
      }
  }
}

function generateFileName(extension) {
  const { year, day, part } = state.params;
  return `Advent-of-Code-${year}-Day-${day}-Part-${part}.${extension}`;
}

function downloadImage() {
  composedCtx.fillStyle = "rgb(13, 12, 26)";
  composedCtx.fillRect(
    0,
    0,
    composedCtx.canvas.width,
    composedCtx.canvas.height
  );
  composedCtx.drawImage(canvas, 0, 0);
  composedCtx.drawImage(overlayCanvas, 0, 0);

  const url = composedCtx.canvas.toDataURL("image/png");
  const a = document.createElement("a");
  a.href = url;
  a.download = generateFileName("png");
  a.click();
}

document.body.addEventListener("keyup", async (e) => {
  switch (e.key) {
    case "Escape":
      window.location = "..";
      break;
    case "Enter":
      toggleFullScreen();
      break;
    case "i": // Image.
      downloadImage();
      break;
    case "p": // Pause.
    case " ":
      await togglePause();
      break;
    case "r": // Restart.
      reloadWithParameters({ download: "" });
      break;
    case "v": // Video.
      reloadWithParameters({
        download: true,
        aspectRatio: state.aspectRatio ? state.aspectRatio : "",
      });
      break;
  }
});

document.documentElement.addEventListener("click", togglePause);
document.documentElement.addEventListener("dblclick", toggleFullScreen);

// https://web.dev/device-pixel-content-box/
function isDevicePixelContentBoxSupported() {
  return new Promise((resolve) => {
    const ro = new ResizeObserver((entries) => {
      resolve(entries.every((entry) => "devicePixelContentBoxSize" in entry));
      ro.disconnect();
    });
    ro.observe(document.body, { box: ["device-pixel-content-box"] });
  }).catch(() => false);
}

function saveContextState(ctx) {
  const props = [
    "strokeStyle",
    "fillStyle",
    "globalAlpha",
    "lineWidth",
    "lineCap",
    "lineJoin",
    "miterLimit",
    "lineDashOffset",
    "shadowOffsetX",
    "shadowOffsetY",
    "shadowBlur",
    "shadowColor",
    "globalCompositeOperation",
    "font",
    "textAlign",
    "textBaseline",
    "direction",
    "imageSmoothingEnabled",
  ];
  const state = {};
  for (let prop of props) state[prop] = ctx[prop];
  return state;
}

function restoreContextState(ctx, state) {
  for (const [key, value] of Object.entries(state)) {
    ctx[key] = value;
  }
}

setTimeout(async () => {
  const devicePixelContentBoxSupported = await isDevicePixelContentBoxSupported();
  const observerOptions = devicePixelContentBoxSupported
    ? { box: ["device-pixel-content-box"] }
    : {};

  let resizeCount = 0;

  new ResizeObserver((entries) => {
    resizeCount++;

    // Save a copy of the main canvas:
    composedCanvas.width = canvas.width;
    composedCtx.clearRect(
      0,
      0,
      composedCtx.canvas.width,
      composedCtx.canvas.height
    );
    composedCtx.drawImage(ctx.canvas, 0, 0);

    const newWidth = devicePixelContentBoxSupported
      ? entries[0].devicePixelContentBoxSize[0].inlineSize
      : canvas.clientWidth * window.devicePixelRatio;
    const newHeight = devicePixelContentBoxSupported
      ? entries[0].devicePixelContentBoxSize[0].blockSize
      : canvas.clientHeight * window.devicePixelRatio;

    // Resize canvas and restore context:
    const savedState = saveContextState(ctx);
    ctx.canvas.width = newWidth;
    ctx.canvas.height = newHeight;
    restoreContextState(ctx, savedState);

    // Paint the old copy (scaled):
    ctx.resetTransform();
    ctx.setTransform(
      ctx.canvas.width / composedCanvas.width,
      0,
      0,
      ctx.canvas.height / composedCanvas.height,
      0,
      0
    );
    ctx.drawImage(composedCanvas, 0, 0);
    ctx.setTransform(canvas.width, 0, 0, canvas.width, 0, 0);

    // Setup the correct transform for future painting:
    const savedOverlayState = saveContextState(overlayCtx);
    overlayCtx.canvas.width = newWidth;
    overlayCtx.canvas.height = newHeight;
    overlayCtx.setTransform(canvas.width, 0, 0, canvas.width, 0, 0);
    restoreContextState(overlayCtx, savedOverlayState);
    if (window.renderer) {
      window.renderer.renderStatusText();
    }

    // TODO: Only have layer canvas if used, compose canvas if recording.
    composedCanvas.width = canvas.width;
    composedCanvas.height = canvas.height;

    if (resizeCount == 1) {
      if (state.params.download) {
        setTimeout(sendMessageToWorker, 500);
      } else {
        sendMessageToWorker();
      }
    }
  }).observe(canvas, observerOptions);
}, 0);

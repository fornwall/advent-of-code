import { AudioPlayer } from "./audio-player.js";
// import CanvasRecorder from "./CanvasRecorder-mp4wasm.js";

const visualizerWorker = new Worker(
  new URL("./worker-visualizer.js", import.meta.url),
  {
    name: "visualizer",
    type: "module",
  }
);

const rendering = document.getElementById("rendering");
const progress = document.getElementById("progress");
const show = document.getElementById("show");
const stepDisplay = document.getElementById("stepDisplay");
let svg = null;
let playInterval = null;

const state = {
  params: {},
  audioPlayer: new AudioPlayer("bounce.mp4", "pop.mp4"),
};

const hash = location.hash.substring(1);
for (const part of hash.split("&")) {
  const [key, value] = part.split("=");
  state.params[key] = decodeURIComponent(value);
}

visualizerWorker.onmessage = (message) => {
  if ("errorMessage" in message.data) {
    window.alert(message.data.errorMessage);
    window.location = "..";
  } else if (message.data.done) {
    console.log(
      "SVG size: " +
        new Intl.NumberFormat().format(message.data.answer.length) +
        " bytes"
    );

    const { year, day, part } = state.params;
    document.getElementById(
      "spinner"
    ).innerHTML = `<h1 style="text-align: center;">Advent of Code ${year}<br/>Day ${day}, part ${part}</h1>`;

    async function onClick() {
      console.log("onClick....");
      document.getElementById("spinner").remove();
      document.documentElement.removeEventListener("click", onClick);
      rendering.innerHTML = message.data.answer;

      svg = rendering.querySelector("svg");
      svg.setAttribute("focusable", "false");

      progress.max = svg.dataset.steps;

      rendering.children[0].setAttribute("width", "100%");
      rendering.children[0].setAttribute("height", "100%");
      rendering.querySelectorAll("script").forEach((e) => eval(e.textContent));

      show.style.display = "block";
      togglePause();
      await toggleFullScreen();
    }
    document.documentElement.addEventListener("click", onClick);
  }
};

async function toggleFullScreen() {
  if (document.fullscreenElement) {
    // Do nothing.
    // document.exitFullscreen();
  } else {
    if (document.documentElement.requestFullscreen) {
      document.documentElement.requestFullscreen();
    } else if (document.documentElement.webkitRequestFullscreen) {
      document.documentElement.webkitRequestFullscreen();
    }
    if (svg) {
      const viewBox = svg.getAttribute("viewBox").split(" ");
      if (parseInt(viewBox[2]) > parseInt(viewBox[3])) {
        try {
          await window.screen.orientation.lock("landscape-primary");
        } catch (_e) {
          // Silently ignore.
        }
      }
    }
  }
}

function generateFileName(extension) {
  const { year, day, part } = state.params;
  return `advent-of-code-${year}-day-${day}-part-${part}.${extension}`;
}

function downloadImage() {
  const svg = rendering.innerHTML;
  const blob = new Blob([svg.toString()]);
  const a = document.createElement("a");
  a.href = window.URL.createObjectURL(blob);
  a.download = generateFileName("svg");
  a.click();
}

document.body.addEventListener("keyup", async (e) => {
  switch (e.key) {
    case "f":
    case "Enter":
      await toggleFullScreen();
      break;
    case "d": // Download.
      downloadImage();
      break;
    case "p": // Pause.
    case " ":
      togglePause();
      break;
    case "r": // Restart.
      progress.value = 0;
      if (!playInterval) togglePause();
      break;
  }
});

function togglePause() {
  console.log("Start of togglePause..");
  if (playInterval) {
    console.log("Clearing interval...");
    clearInterval(playInterval);
    playInterval = null;
  } else {
    console.log("Starting interval...");
    playInterval = setInterval(() => {
      if (progress.value == progress.max) {
        togglePause();
      } else {
        progress.value = parseInt(progress.value) + 1;
        progress.dispatchEvent(new Event("input"));
      }
    }, 50);
  }
}

progress.addEventListener("input", () => {
  stepDisplay.textContent = `Step ${progress.value} / ${progress.max}`;
  svg.style.setProperty("--step", progress.value);
  if (window.onNewStep) {
    window.onNewStep(parseInt(progress.value));
  }
});

progress.addEventListener("click", () => {
  if (playInterval) togglePause();
});

progress.addEventListener("touchstart", () => {
  if (playInterval) togglePause();
});

function sendMessageToWorker() {
  const { year, day, part, input } = state.params;
  visualizerWorker.postMessage({ year, day, part, input });
}

sendMessageToWorker();

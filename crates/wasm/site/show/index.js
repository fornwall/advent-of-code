import { AudioPlayer } from "./audio-player.js";
// import CanvasRecorder from "./CanvasRecorder-mp4wasm.js";

const visualizerWorker = new Worker(
  new URL("./worker-visualizer.js", import.meta.url),
  {
    name: "visualizer",
    type: "module",
  }
);

const spinner = document.getElementById("spinner");
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

visualizerWorker.onmessage = (message) => {
  if ("errorMessage" in message.data) {
    console.error("Input error:", message.data.errorMessage);
    spinner.innerHTML = `<h1 style="text-align: center;">Input error: ${message.data.errorMessage}</h1>`;
  } else if (message.data.done) {
    console.log(
      "SVG size: " +
        new Intl.NumberFormat().format(message.data.answer.length) +
        " bytes"
    );

    const { year, day, part } = state.params;
    spinner.innerHTML = `<h1 style="text-align: center;">Advent of Code ${year}<br/>Day ${day}, part ${part}<br/><br/>Click to start</h1>`;

    async function onClick() {
      spinner.style.display = "none";
      document.documentElement.removeEventListener("click", onClick);
      rendering.innerHTML = message.data.answer;

      svg = rendering.querySelector("svg");
      svg.setAttribute("focusable", "false");

      progress.max = svg.dataset.steps;
      state.stepDuration = parseInt(svg.dataset.stepDuration);

      rendering.querySelectorAll("script").forEach((el) => {
        try {
          eval(el.textContent);
        } catch (e) {
          console.error(
            "Error evaluating script: " + e.message,
            el.textContent
          );
          window.alert("Error in script - see console logs");
        }
      });

      rendering.addEventListener("click", () => {
        togglePause();
      });

      rendering.addEventListener("dblclick", () => {
        toggleFullScreen();
      });

      show.style.display = "flex";
      setCurrentStep(0);
      setTimeout(() => {
        if (!playInterval) togglePause();
      }, 1000);
      await toggleFullScreen();
    }
    document.documentElement.addEventListener("click", onClick);
  }
};

async function toggleFullScreen() {
  if (document.fullscreenElement) {
    // Do nothing.
    document.exitFullscreen();
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

document.body.addEventListener("keydown", async (e) => {
  const multiplier = e.shiftKey || e.altKey || e.metaKey ? 10 : 1;
  switch (e.key) {
    case "ArrowLeft":
      if (playInterval) togglePause();
      changeCurrentValue(-1 * multiplier);
      break;
    case "ArrowRight":
      if (playInterval) togglePause();
      changeCurrentValue(1 * multiplier);
      break;
    case "f":
      await toggleFullScreen();
      break;
    case "d":
      downloadImage();
      break;
    case "Enter":
    case "p":
    case " ":
      togglePause();
      break;
    case "r":
      setCurrentStep(0);
      if (!playInterval) togglePause();
      break;
  }
});

function setCurrentStep(value) {
  progress.value = value;
  progress.dispatchEvent(new Event("input"));
}

function changeCurrentValue(change) {
  setCurrentStep(parseInt(progress.value) + change);
}

function togglePause() {
  if (playInterval) {
    clearInterval(playInterval);
    playInterval = null;
  } else {
    if (progress.value == progress.max) {
      setCurrentStep(0);
    }
    playInterval = setInterval(() => {
      if (progress.value == progress.max) {
        togglePause();
      } else {
        changeCurrentValue(1);
      }
    }, state.stepDuration);
  }
}

progress.addEventListener("input", () => {
  stepDisplay.innerHTML =
    "&nbsp;".repeat(progress.max.length - progress.value.length) +
    `${progress.value}/${progress.max}`;
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

document.documentElement.ondragover = (dragOverEvent) => {
  dragOverEvent.preventDefault();
  dragOverEvent.dataTransfer.dropEffect = Array.from(
    dragOverEvent.dataTransfer.items
  ).some((item) => item.type.match("^text/plain"))
    ? "copy"
    : "none";
};

document.documentElement.ondrop = async (dropEvent) => {
  dropEvent.preventDefault();
  for (const item of dropEvent.dataTransfer.items) {
    if (item.kind == "string" && item.type.match("^text/plain")) {
      item.getAsString((s) => sendMessageToWorker(s));
    } else if (item.kind == "file" && item.type.match("^text/plain")) {
      sendMessageToWorker(await item.getAsFile().text());
    }
  }
};

function revertDisplay() {
  rendering.innerHTML = "";
  progress.value = 0;
  show.style.display = "none";
  spinner.style.display = "flex";
  spinner.innerHTML = `<img
        id="spinnerImage"
        alt="Loading…"
        src="/static/spinner.svg"
        style="z-index: 100; max-width: 100%; max-height: 100%"
      />`;
}

function sendMessageToWorker(newInput) {
  if (newInput) {
    if (playInterval) togglePause();
    state.params.input = newInput;
    revertDisplay();
  }
  const { year, day, part, input } = state.params;
  visualizerWorker.postMessage({ year, day, part, input });
}

async function onLoad() {
  revertDisplay();

  const searchParams = new URLSearchParams(window.location.search);
  for (const [key, value] of searchParams) {
    state.params[key] = decodeURIComponent(value);
  }

  if (!state.params["input"]) {
    const tests = await (
      await fetch("https://fornwall.net/aoc/tests.json")
    ).json();
    state.params["input"] = tests.years
      .find((y) => y.year == state.params["year"])
      .days.find((d) => d.day == state.params["day"])["input"];
  }
  sendMessageToWorker();
}

onLoad();

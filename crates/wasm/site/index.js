import gistMapping from "./gist-mapping.json";
const worker = new Worker("./worker.js", { name: "solver" });

const yearElement = document.getElementById("year");
const dayElement = document.getElementById("day");
const partElement = document.getElementById("part");
const inputElement = document.getElementById("input");

const outputElement = document.getElementById("output");

const apiExecutionTimeElement = document.getElementById("api-execution-time");
const wasmExecutionTimeElement = document.getElementById("wasm-execution-time");

const runWasmElement = document.getElementById("run-wasm");
const runApiElement = document.getElementById("run-api");
const showElement = document.getElementById("run-visualizer");

worker.onmessage = (e) => {
  if ("wasmWorking" in e.data) {
    if (!e.data.wasmWorking) {
      runWasmElement.disabled = true;
      runWasmElement.title = "Wasm is not working - check console logs";
      showElement.disabled = true;
    }
  } else {
    const { isError, output, wasm, executionTime } = e.data;
    const runButton = wasm ? runWasmElement : runApiElement;
    runButton.classList.remove("in-progress");
    runButton.disabled = false;
    showMessage(output, isError, wasm, executionTime);
  }
};

function showMessage(message, isError, wasm, executionTime) {
  const executionTimeElement = wasm
    ? wasmExecutionTimeElement
    : apiExecutionTimeElement;
  executionTimeElement.textContent = `${Math.round(executionTime)}`;

  outputElement.classList.remove("alert-info");
  if (isError) {
    outputElement.classList.add("alert-danger");
    outputElement.classList.remove("alert-success");
  } else {
    outputElement.classList.add("alert-success");
    outputElement.classList.remove("alert-danger");
  }
  outputElement.textContent = message;
  outputElement.scrollIntoView();
  outputElement.classList.add("blink");
  outputElement.focus();
}

function execute(wasm) {
  partElement.setCustomValidity(
    dayElement.value == 25 && partElement.value == 2
      ? "Day 25 has no second part."
      : ""
  );

  if (document.querySelector("form").reportValidity()) {
    const runButton = wasm ? runWasmElement : runApiElement;
    runButton.disabled = true;
    runButton.classList.add("in-progress");
    outputElement.classList.remove("blink");
    const [year, day, part, input] = [
      yearElement.value,
      dayElement.value,
      partElement.value,
      inputElement.value,
    ];
    worker.postMessage({ year, day, part, input, wasm });
  }
}

function isVisualisationEnabled() {
  return (
    !runWasmElement.disabled &&
    !!gistMapping?.[yearElement.value]?.[dayElement.value]?.["visualization"]
  );
}

function visualize() {
  const [year, day, part, input] = [
    yearElement.value,
    dayElement.value,
    partElement.value,
    inputElement.value,
  ];
  const visualizerUrl = `/show/#year=${year}&day=${day}&part=${part}&input=${encodeURIComponent(
    input
  )}`;
  window.location = visualizerUrl;
}

function storeForm() {
  window.localStorage.setItem(
    "problem",
    JSON.stringify({
      year: yearElement.value,
      day: dayElement.value,
      part: partElement.value,
      input: inputElement.value,
    })
  );
}

async function clipboardReadMayWork() {
  if (navigator.clipboard && navigator.clipboard.readText) {
    if (navigator.permissions) {
      const permission = await navigator.permissions.query({
        name: "clipboard-read",
      });
      return permission.state !== "denied";
    } else {
      return true;
    }
  } else {
    return false;
  }
}

runApiElement.addEventListener("click", () => execute(false));
runWasmElement.addEventListener("click", () => execute(true));

showElement.addEventListener("click", visualize);
showElement.disabled = !isVisualisationEnabled();

[yearElement, dayElement].forEach((element) =>
  element.addEventListener("change", () => {
    showElement.disabled = !isVisualisationEnabled();
  })
);

[yearElement, dayElement, partElement, inputElement].forEach((element) =>
  element.addEventListener("change", storeForm)
);

window.addEventListener("pageshow", () => {
  const problemString = window.localStorage.getItem("problem");
  if (problemString) {
    try {
      const problem = JSON.parse(problemString);
      yearElement.value = problem.year;
      dayElement.value = problem.day;
      partElement.value = problem.part;
      if (problem.input) inputElement.value = problem.input;

      dayElement.dispatchEvent(new Event("change"));
    } catch (error) {
      console.error(error);
    }
  }
});

document.getElementById("open-input").addEventListener("click", () => {
  const link = `https://adventofcode.com/${yearElement.value}/day/${dayElement.value}/input`;
  window.open(link);
});

const savedInterval = { value: null };
document.getElementById("output").addEventListener("click", (event) => {
  if (savedInterval.value) {
    clearTimeout(savedInterval.value);
  }
  navigator.clipboard.writeText(event.target.textContent);
  event.target.classList.add("copied");
  savedInterval.value = setTimeout(
    () => event.target.classList.remove("copied"),
    2000
  );
});

clipboardReadMayWork().then((enabled) => {
  const pasteButton = document.getElementById("paste");
  if (enabled) {
    pasteButton.addEventListener("click", async () => {
      inputElement.value = await navigator.clipboard.readText();
      storeForm();
    });
  } else {
    pasteButton.disabled = true;
  }
});

document.getElementById("open-playground").addEventListener("click", () => {
  const gistId = gistMapping?.[yearElement.value]?.[dayElement.value]?.["gist"];
  if (gistId) {
    const link = `https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=${gistId}`;
    window.open(link);
  } else {
    alert("Not available yet!");
  }
});

import gistMapping from "./gist-mapping.json";

const yearElement = document.getElementById("year");
const dayElement = document.getElementById("day");
const partElement = document.getElementById("part");
const inputElement = document.getElementById("input");
const outputElement = document.getElementById("output");
const executionTimeElement = document.getElementById("execution-time");
const runButton = document.getElementById("solve-button");
const showElement = document.getElementById("run-visualizer");

const workers = {
  api: null,
  wasm: null,
};

function reloadApiWorker() {
  if (workers.api) workers.api.terminate();
  workers.api = new Worker(new URL("./worker-api.js", import.meta.url), {
    name: "api-solver",
  });
  workers.api.onmessage = onWorkerMessage;
}
function reloadWasmWorker() {
  if (workers.wasm) workers.wasm.terminate();
  workers.wasm = new Worker(new URL("./worker-wasm.js", import.meta.url), {
    name: "wasm-solver",
  });
  workers.wasm.onmessage = onWorkerMessage;
}

reloadApiWorker();
reloadWasmWorker();

const wasmWorking = { value: true };
const currentProblem = {
  year: 0,
  day: 0,
  part: 0,
  input: null,
  output: null,
  isInternalError: false,
};

function onWorkerMessage(e) {
  if ("wasmWorking" in e.data) {
    if (!e.data.wasmWorking) {
      wasmWorking.value = false;
    }
  } else {
    const {
      worker,
      isError,
      isInternalError,
      output,
      executionTime,
      year,
      day,
      part,
      input,
    } = e.data;

    const secondAnswer =
      currentProblem.year === year &&
      currentProblem.day == day &&
      currentProblem.part === part &&
      currentProblem.input == input;

    let extraOutput = "";

    if (isInternalError) {
      if (currentProblem.isInternalError) {
        // Both workers have failed with internal errors.
        extraOutput = "\n" + currentProblem.output;
      } else if (secondAnswer) {
        // This was the second answer, but first one was ok - ignore.
        return;
      } else {
        // If this was the first worker, await other one which may have better luck.
        currentProblem.isInternalError = true;
        currentProblem.output = output;
        return;
      }
    }

    if (secondAnswer) {
      // This is the second message about this problem.
      return;
    } else if (worker == "api") {
      // We got a API response first and can abort the Wasm worker.
      reloadWasmWorker();
    } else if (worker == "wasm") {
      // We got a Wasm response first and can abort the API worker.
      reloadApiWorker();
    }

    currentProblem.isInternalError = false;
    currentProblem.year = year;
    currentProblem.day = day;
    currentProblem.part = part;
    currentProblem.input = input;

    runButton.classList.remove("in-progress");
    runButton.disabled = false;

    const roundedTime = Math.round(executionTime);
    executionTimeElement.textContent = `${
      roundedTime == 0
        ? executionTime.toFixed(2)
        : roundedTime.toLocaleString("en")
    } ms`;

    outputElement.classList.remove("alert-info");
    if (isError) {
      outputElement.classList.add("alert-danger");
      outputElement.classList.remove("alert-success");
    } else {
      outputElement.classList.add("alert-success");
      outputElement.classList.remove("alert-danger");
    }
    outputElement.textContent =
      (isInternalError ? "⚠ Internal Error ⚠\n\n" : "") +
      output +
      extraOutput;
    outputElement.scrollIntoView();
    outputElement.classList.add("blink");
    outputElement.focus();
  }
}

function execute() {
  partElement.setCustomValidity(
    dayElement.value == 25 && partElement.value == 2
      ? "Day 25 has no second part."
      : "",
  );

  if (document.querySelector("form").reportValidity()) {
    runButton.disabled = true;
    runButton.classList.add("in-progress");
    outputElement.classList.remove("blink");
    const [year, day, part, input] = [
      yearElement.value,
      dayElement.value,
      partElement.value,
      inputElement.value,
    ];

    currentProblem.input = null;
    currentProblem.output = null;
    currentProblem.isInternalError = false;
    workers.api.postMessage({ year, day, part, input });
    workers.wasm.postMessage({ year, day, part, input });
    outputElement.classList.remove("alert-danger");
    outputElement.classList.remove("alert-success");
    outputElement.classList.add("alert-info");
    outputElement.textContent = "The answer is being computed...";
  }
}

function isVisualisationEnabled() {
  return (
    wasmWorking.value &&
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
  const inputParam = input ? `&input=${encodeURIComponent(input)}` : "";
  const visualizerUrl = `/show/?year=${year}&day=${day}&part=${part}${inputParam}`;
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
    }),
  );
}

function setInputText(input) {
  inputElement.value = input.trim();
  storeForm();
  runButton.dispatchEvent(new Event("click"));
}

const savedInterval = { value: null };
function notifyOutputCopied() {
  if (savedInterval.value) {
    clearTimeout(savedInterval.value);
  }
  outputElement.classList.add("copied");
  savedInterval.value = setTimeout(
    () => outputElement.classList.remove("copied"),
    2000,
  );
}

runButton.addEventListener("click", () => execute());

showElement.addEventListener("click", visualize);
showElement.disabled = !isVisualisationEnabled();

[yearElement, dayElement].forEach((element) =>
  element.addEventListener("change", () => {
    showElement.disabled = !isVisualisationEnabled();
  }),
);

[yearElement, dayElement, partElement, inputElement].forEach((element) =>
  element.addEventListener("change", storeForm),
);

globalThis.addEventListener("pageshow", () => {
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

document.getElementById("open-problem").addEventListener("click", () => {
  let link = `https://adventofcode.com/${yearElement.value}/day/${dayElement.value}`;
  if (partElement.value == 2) link += "#part2";
  window.open(link);
});

outputElement.addEventListener("click", (event) => {
  navigator.clipboard.writeText(event.target.textContent);
  notifyOutputCopied();
});

const pasteButton = document.getElementById("paste");
if (navigator?.clipboard?.readText) {
  pasteButton.addEventListener("click", async () => {
    setInputText(await navigator.clipboard.readText());
  });
} else {
  pasteButton.disabled = true;
}

document.getElementById("open-playground").addEventListener("click", () => {
  const gistId = gistMapping?.[yearElement.value]?.[dayElement.value]?.["gist"];
  if (gistId) {
    const link = `https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=${gistId}`;
    window.open(link);
  } else {
    alert("Not available yet!");
  }
});

outputElement.draggable = true;

outputElement.addEventListener("dragstart", (dragEvent) => {
  if (dragEvent.target.tagName == "TEXTAREA") {
    return;
  }
  dragEvent.dataTransfer.dropEffect = "copy";
  dragEvent.dataTransfer.effectAllowed = "copy";
  dragEvent.dataTransfer.setDragImage(outputElement, 0, 0);
  dragEvent.dataTransfer.setData(
    "text/plain",
    outputElement.textContent.trim(),
  );
});

document.documentElement.ondragover = (dragOverEvent) => {
  dragOverEvent.preventDefault();
  dragOverEvent.dataTransfer.dropEffect = Array.from(
    dragOverEvent.dataTransfer.items,
  ).some((item) => item.type.match("^text/plain"))
    ? "copy"
    : "none";
};

document.documentElement.ondrop = async (dropEvent) => {
  dropEvent.preventDefault();
  for (const item of dropEvent.dataTransfer.items) {
    if (item.kind == "string" && item.type.match("^text/plain")) {
      item.getAsString((s) => setInputText(s));
    } else if (item.kind == "file" && item.type.match("^text/plain")) {
      setInputText(await item.getAsFile().text());
    }
  }
};

document.addEventListener("paste", (event) => {
  if (event.target.tagName !== "TEXTAREA") {
    event.preventDefault();
    setInputText(event.clipboardData.getData("text/plain"));
  }
});

document.addEventListener("copy", (event) => {
  if (event.target.tagName !== "TEXTAREA") {
    event.preventDefault();
    event.clipboardData.setData("text/plain", outputElement.textContent.trim());
    notifyOutputCopied();
  }
});

const registerServiceWorker = async () => {
  const digestMessage = async (message) => {
    const msgUint8 = new TextEncoder().encode(message);
    const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map((b) => b.toString(16).padStart(2, "0")).join("");
  };

  if ("serviceWorker" in navigator) {
    try {
      const swResponse = await fetch("service-worker.js?ts=" + Date.now());
      const swText = await swResponse.text();
      const swHash = await digestMessage(swText);
      const registration = await navigator.serviceWorker.register(
        "service-worker.js?hash=" + swHash,
        {
          scope: "/",
        },
      );

      // https://whatwebcando.today/articles/handling-service-worker-updates/
      registration.addEventListener("updatefound", () => {
        if (registration.installing) {
          registration.installing.addEventListener("statechange", () => {
            if (registration.waiting && navigator.serviceWorker.controller) {
              window.location.reload();
            }
          });
        }
      });
    } catch (error) {
      console.error(`Registration failed with ${error}`);
    }
  }
};

registerServiceWorker();

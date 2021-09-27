import gistMapping from "./gist-mapping.json";
const worker = new Worker("./worker.js", { name: "solver" });

const yearElement = document.getElementById("year");
const dayElement = document.getElementById("day");
const partElement = document.getElementById("part");
const inputElement = document.getElementById("input");

const outputElement = document.getElementById("output");

const executionTimeElement = document.getElementById("execution-time");

const runButton = document.getElementById("solve-button");
const showElement = document.getElementById("run-visualizer");
const wasmWorking = { value: true };

worker.onmessage = (e) => {
  if ("wasmWorking" in e.data) {
    if (!e.data.wasmWorking) {
      wasmWorking.value = false;
    }
  } else {
    const { isError, output, executionTime } = e.data;
    runButton.classList.remove("in-progress");
    runButton.disabled = false;
    showMessage(output, isError, executionTime);
  }
};

function showMessage(message, isError, executionTime) {
  executionTimeElement.textContent = `${Math.round(executionTime)} ms`;

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

function execute() {
  partElement.setCustomValidity(
    dayElement.value == 25 && partElement.value == 2
      ? "Day 25 has no second part."
      : ""
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
    worker.postMessage({ year, day, part, input });
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

runButton.addEventListener("click", () => execute());

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
function notifyOutputCopied() {
  if (savedInterval.value) {
    clearTimeout(savedInterval.value);
  }
  outputElement.classList.add("copied");
  savedInterval.value = setTimeout(
    () => outputElement.classList.remove("copied"),
    2000
  );
}
document.getElementById("output").addEventListener("click", (event) => {
  navigator.clipboard.writeText(event.target.textContent);
  notifyOutputCopied();
});

clipboardReadMayWork().then((enabled) => {
  const pasteButton = document.getElementById("paste");
  if (enabled) {
    pasteButton.addEventListener("click", async () => {
      inputElement.value = await navigator.clipboard.readText();
      storeForm();
      runButton.dispatchEvent(new Event("click"));
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

window.addEventListener("DOMContentLoaded", () => {
  const element = document.documentElement;
  element.draggable = true;
  element.addEventListener("dragstart", (dragEvent) => {
    if (dragEvent.target.tagName == "TEXTAREA") {
      return;
    }
    dragEvent.dataTransfer.dropEffect = "copy";
    dragEvent.dataTransfer.effectAllowed = "copy";
    dragEvent.dataTransfer.setDragImage(outputElement, 0, 0);
    dragEvent.dataTransfer.setData(
      "text/plain",
      outputElement.textContent.trim()
    );
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
        item.getAsString((s) => (inputElement.value = s));
      } else if (item.kind == "file" && item.type.match("^text/plain")) {
        document.getElementById("input").value = (
          await item.getAsFile().text()
        ).trim();
      }
    }
  };

  document.addEventListener("paste", (event) => {
    inputElement.value = event.clipboardData.getData("text/plain");
    storeForm();
    runButton.dispatchEvent(new Event("click"));
  });
  document.addEventListener("copy", (event) => {
    event.clipboardData.setData("text/plain", outputElement.textContent.trim());
    event.preventDefault();
    notifyOutputCopied();
  });
});

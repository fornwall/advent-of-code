'use strict';
const worker = new Worker("./worker.js", { name: "solver" });

const input_instructions_element = document.getElementById('input-instructions');
const run_wasm_element = document.getElementById('run-wasm');
const run_api_element = document.getElementById('run-api');
const year_element = document.getElementById('year');
const day_element = document.getElementById('day');
const part_element = document.getElementById('part');
const input_element = document.getElementById('input');
const output_element = document.getElementById('output');
const executionTime_element = document.getElementById('executionTime');

worker.onmessage = (e) => {
  if ('wasmWorking' in e.data) {
    if (!e.data.wasmWorking) {
        run_wasm_element.disabled = true;
        run_wasm_element.classList.add('unusable');
        run_wasm_element.title = 'Wasm is not working - check console logs';
    }
  } else {
      const { isError, output, wasm, executionTime } = e.data;
      (wasm ? run_wasm_element : run_api_element).disabled = false;
      showMessage(output, isError, wasm, executionTime);
  }
}

function showMessage(message, isError, wasm, executionTime) {
  executionTime_element.textContent = ` (${wasm ? 'Wasm' : 'API'} in ${Math.round(executionTime)} ms)`;
  if (isError) {
    output_element.classList.add('error');
  } else {
    output_element.classList.remove('error');
  }
  output_element.textContent = message;
  output_element.scrollIntoView();
  output_element.classList.add('blink');
  output_element.focus();
}

function execute(wasm) {
  if (document.querySelector("form").reportValidity()) {
      (wasm ? run_wasm_element : run_api_element).disabled = true;
      output_element.classList.remove('blink');
      const [year, day, part, input] = [year_element.value, day_element.value, part_element.value, input_element.value];
      worker.postMessage({year, day, part, input, wasm});
  }
}

function updateInputLink() {
  const link = `adventofcode.com/${year_element.value}/day/${day_element.value}/input`;
  input_instructions_element.innerHTML = `Your input is at <a href="https://${link}">${link}</a>.`;
}

window.addEventListener('pageshow', () => {
    if (window.localStorage) {
        const problemString = window.localStorage.getItem("problem");
        if (problemString) {
            try {
                const problem = JSON.parse(problemString)
                year_element.value = problem.year;
                day_element.value = problem.day;
                part_element.value = problem.part;
            } catch (error) {
                log.error(error);
            }
        }
    }
    updateInputLink();
});

async function run() {
  run_api_element.addEventListener("click", (event) => execute(false));
  run_wasm_element.addEventListener("click", (event) => execute(true));

  [year_element, day_element, part_element].forEach(element => element.addEventListener('input', () => {
    updateInputLink();
    if (window.localStorage) {
        window.localStorage.setItem('problem', JSON.stringify({year: year_element.value, day: day_element.value, part: part_element.value}));
    }
  }, false));

  if (navigator.clipboard && navigator.clipboard.readText) {
    const pasteButton = document.getElementById('paste');
    pasteButton.classList.remove('hidden');
    document.getElementById('paste').addEventListener('click', async () => {
      try {
        input_element.value = await navigator.clipboard.readText();
      } catch (e) {
        console.log(e);
      }
    }, false);
  }

  if (window.showOpenFilePicker) {
    const readFileButton = document.getElementById("read_file");
    readFileButton.classList.remove("hidden");
    readFileButton.addEventListener("click", async () => {
      try {
        let fileHandle;
        [fileHandle] = await window.showOpenFilePicker();
        const file = await fileHandle.getFile();
        const contents = await file.text();
        document.getElementById('input').value = contents;
      } catch (e) {
        console.log(e);
      }
    });
  }
}

run();

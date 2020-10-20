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
        run_wasm_element.title = 'Wasm is not working - check console logs';
    }
    return;
  }
  const { isError, output, wasm } = e.data;
  run_wasm_element.innerHTML = 'Run Wasm';
  run_wasm_element.disabled = false;
  run_wasm_element.classList.remove('in-progress');
  run_api_element.innerHTML = 'Run API';
  run_api_element.disabled = false;
  run_wasm_element.classList.remove('in-progress');
  showMessage(output, isError, wasm);
}

function showMessage(message, isError, wasm) {
  const executionTime = performance.now() - window.solveStart;
  executionTime_element.textContent = ' (from ' + (wasm?'Wasm':'API') + ' in ' + Math.round(executionTime) + ' ms)';
  if (isError) {
    output_element.classList.add('error');
  } else {
    clearError(false);
  }
  output_element.textContent = message;
  output_element.scrollIntoView();
  output_element.classList.add('blink');
  output_element.focus();
}

function clearError() {
  output_element.innerHTML = '&nbsp;';
  output_element.classList.remove('error');
}

function execute(event, wasm) {
  if (!document.querySelector("form").reportValidity()) {
      return;
  }
  const year = year_element.options[year_element.selectedIndex].value;
  const day = day_element.options[day_element.selectedIndex].value;
  const part = part_element.options[part_element.selectedIndex].value;
  const input = input_element.value;

  window.solveStart = performance.now();
  let button = event.target;
  button.disabled = true;
  button.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Running...';
  button.classList.add('in-progress');
  output_element.classList.remove('blink');
  worker.postMessage({year, day, part, input, wasm});
}

function updateInputLink() {
  const link = `adventofcode.com/${year_element.value}/day/${day_element.value}/input`;
  input_instructions_element.innerHTML = `Your input is at <a href="https://${link}">${link}</a>.`;
}

window.addEventListener('pageshow', () => updateInputLink());

async function run() {
  run_api_element.addEventListener("click", (event) => execute(event, false));
  run_wasm_element.addEventListener("click", (event) => execute(event, true));

  [year_element, day_element, part_element, input_element].forEach(element => element.addEventListener('input', (event) => {
    //element.setCustomValidity('');
    updateInputLink();
  }, false));

  if (navigator.clipboard) {
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

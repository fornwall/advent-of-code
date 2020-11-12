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
const api_execution_time = document.getElementById('api-execution-time');
const wasm_execution_time = document.getElementById('wasm-execution-time');

function disableButton(buttonToDisable) {
  buttonToDisable.disabled = true;
  buttonToDisable.classList.add('unusable');
}

worker.onmessage = (e) => {
  if ('wasmWorking' in e.data) {
    if (!e.data.wasmWorking) {
      disableButton(run_wasm_element);
      run_wasm_element.title = 'Wasm is not working - check console logs';
    }
  } else {
    const { isError, output, wasm, executionTime } = e.data;
    (wasm ? run_wasm_element : run_api_element).disabled = false;
    showMessage(output, isError, wasm, executionTime);
  }
}

function showMessage(message, isError, wasm, executionTime) {
  const execution_time = wasm ? wasm_execution_time : api_execution_time;
  execution_time.textContent = `${Math.round(executionTime)} ms`;
  output_element.classList.remove('alert-info');
  if (isError) {
    output_element.classList.add('alert-danger');
    output_element.classList.remove('alert-success');
  } else {
    output_element.classList.add('alert-success');
    output_element.classList.remove('alert-danger');
  }
  output_element.textContent = message;
  output_element.scrollIntoView();
  output_element.classList.add('blink');
  output_element.focus();
}

function execute(wasm) {
  part_element.setCustomValidity((day_element.value == 25 && part_element.value == 2) ? 'Day 25 has no second part.' : '');

  if (document.querySelector("form").reportValidity()) {
    (wasm ? run_wasm_element : run_api_element).disabled = true;
    output_element.classList.remove('blink');
    const [year, day, part, input] = [year_element.value, day_element.value, part_element.value, input_element.value];
    worker.postMessage({ year, day, part, input, wasm });
  }
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
        console.error(error);
      }
    }
  }
});

async function clipboardMayWork() {
  if (navigator.clipboard && navigator.clipboard.readText) {
    if (navigator.permissions) {
      const permission = await navigator.permissions.query({ name: 'clipboard-read' });
      return permission.state !== 'denied';
    } else {
      return true;
    }
  } else {
    return false;
  }
}

function run() {
  run_api_element.addEventListener("click", () => execute(false));
  run_wasm_element.addEventListener("click", () => execute(true));

  const state = {};

  document.getElementById('open-playground').addEventListener("click", () => {
    const gist_id = state['mapping']?.[year_element.value]?.[day_element.value];
    if (gist_id) {
      const link = `https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=${gist_id}`;
      window.open(link)
    } else {
      alert('Not available yet!');
    }
  });

  [year_element, day_element, part_element].forEach(element => element.addEventListener('input', () => {
    if (window.localStorage) {
      window.localStorage.setItem('problem', JSON.stringify({ year: year_element.value, day: day_element.value, part: part_element.value }));
    }
  }, false));

  document.getElementById('open-input').addEventListener('click', () => {
    const link = `https://adventofcode.com/${year_element.value}/day/${day_element.value}/input`;
    window.open(link)
  }, false);

  document.getElementById('output').addEventListener('click', (event) => {
    if (state['copiedTimeout']) {
      clearTimeout(state['copiedTimeout']);
    }
    navigator.clipboard.writeText(event.target.textContent);
    event.target.classList.add('copied');
    state['copiedTimeout'] = setTimeout(() => event.target.classList.remove('copied'), 2000);
  });

  clipboardMayWork().then((enabled) => {
    const pasteButton = document.getElementById('paste');
    if (enabled) {
      pasteButton.addEventListener('click', async () => {
        try {
          input_element.value = await navigator.clipboard.readText();
        } catch (e) {
          console.log(e);
        }
      }, false);
    } else {
      disableButton(pasteButton);
    }
  });

  fetch('gist-mapping.json')
    .then(response => response.json())
    .then(data => state['mapping'] = data);
}

run();

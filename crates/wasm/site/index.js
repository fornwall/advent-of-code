import init, { solve } from './advent_of_code_wasm.js';

const year_element = document.getElementById('year');
const day_element = document.getElementById('day');
const part_element = document.getElementById('part');
const input_element = document.getElementById('input');
const output_element = document.getElementById('output');

function showMessage(message, isError) {
    if (isError) {
        output_element.classList.add('error');
        input_element.setCustomValidity(message);
        document.querySelector("form").reportValidity();
    } else {
        clearError(false);
    }
    output_element.textContent = message;
    output_element.scrollIntoView();
    output_element.classList.add('blink');
}

function clearError(removeBlink) {
    input_element.setCustomValidity('');
    output_element.innerHTML = '&nbsp;';
    output_element.classList.remove('error');
    if (removeBlink) output_element.classList.remove('blink');
}

async function run() {
    try {
	    await init();
	} catch (e) {
	    console.warn(e);
	    document.getElementById('run').disabled = true;
	    return;
	}

	[day_element, part_element, input_element].forEach(element => element.addEventListener('input', () => {
	  clearError(true);
	}, false));

	document.querySelector("form").addEventListener("submit", (event) => {
	   event.preventDefault();
	   const year = year_element.options[year_element.selectedIndex].value;
	   const day = day_element.options[day_element.selectedIndex].value;
	   const part = part_element.options[part_element.selectedIndex].value;
	   const input = input_element.value;

	   try {
	      const message = solve(year, day, part, input);
	      showMessage(message);
	   } catch (e) {
		  console.log(e);
	      showMessage(e.message, true);
	   }
	});

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
				// Ignore user aborting request.
			}
		});
	}
}

run();

import * as wasm from "advent_of_code_rs";

const day_element = document.getElementById('day');
const part_element = document.getElementById('part');
const input_element = document.getElementById('input');
const output_element = document.getElementById('output');

[day_element, part_element, input_element].forEach(element => element.addEventListener('input', function() {
  output_element.textContent = '';
  output_element.classList.remove('blink');
}, false));

document.getElementById("run_button").addEventListener("click", function() {
   const day = parseInt(day_element.options[day_element.selectedIndex].value);
   const part = parseInt(part_element.options[part_element.selectedIndex].value);
   const input = input_element.value;

   let message;
   try {
      message = wasm.solve(day, part, input);
   } catch (e) {
      console.log(e);
      message = 'ERROR: Invalid input or bug in solution';
   }
   output_element.textContent = message;
   output_element.scrollIntoView();
   output_element.classList.add('blink');
});

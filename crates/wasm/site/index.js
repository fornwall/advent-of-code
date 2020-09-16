import * as wasm from "advent-of-code-wasm";

const year_element = document.getElementById('year');
const day_element = document.getElementById('day');
const part_element = document.getElementById('part');
const input_element = document.getElementById('input');
const output_element = document.getElementById('output');

[day_element, part_element, input_element].forEach(element => element.addEventListener('input', function() {
  output_element.textContent = '';
  output_element.classList.remove('blink');
}, false));

document.getElementById("run_button").addEventListener("click", function() {
   const year = parseInt(year_element.options[year_element.selectedIndex].value);
   const day = parseInt(day_element.options[day_element.selectedIndex].value);
   const part = parseInt(part_element.options[part_element.selectedIndex].value);
   const input = input_element.value;

   let message;
   try {
      message = wasm.solve(year, day, part, input);
   } catch (e) {
      console.log(e);
      message = e.message;
   }
   output_element.textContent = message;
   output_element.scrollIntoView();
   output_element.classList.add('blink');
});

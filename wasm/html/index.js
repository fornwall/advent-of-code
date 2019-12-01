import * as wasm from "advent_of_code_rs";

document.getElementById("run_button").addEventListener("click", function() {
   const day_element = document.getElementById('day');
   const part_element = document.getElementById('part');
   const input_element = document.getElementById('input');
   const output_element = document.getElementById('output');

   const day = parseInt(day_element.options[day_element.selectedIndex].value);
   const part = parseInt(part_element.options[part_element.selectedIndex].value);
   const input = input_element.value;

   input_element.addEventListener('input', function() {
      output_element.textContent = '';
   }, false);


   let message;
   try {
      message = wasm.solve(day, part, input);
   } catch (e) {
      console.log(e);
      message = 'ERROR: Invalid input or bug in solution';
   }
   output_element.textContent = message;
});

import * as wasm from "advent_of_code_rs";

document.getElementById("run_button").addEventListener("click", function() {
   const day_element = document.getElementById('day');
   const part_element = document.getElementById('part');
   const input_element = document.getElementById('input');

   const day = parseInt(day_element.options[day_element.selectedIndex].value);
   const part = parseInt(part_element.options[part_element.selectedIndex].value);
   const input = input_element.value;

   const solution = wasm.solve(day, part, input);
   document.getElementById('output').textContent = solution;
});

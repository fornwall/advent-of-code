import init, { solve } from './show/advent_of_code_wasm.js';

async function run() {
    console.log('before init in visualier');
  await init();
    console.log('init done message im visualier');

  self.onmessage = async (message) => {
    console.log('on message im visualier');
    const { year, day, part, input, canvas } = message.data;

    self.canvas = canvas;
    self.promise = new Promise((resolve, reject) => {
      self.resolve = resolve;
    });

    function render(time) {
      self.resolve();
      self.promise = new Promise((resolve, reject) => {
        self.resolve = resolve;
      });
      requestAnimationFrame(render);
    }
    requestAnimationFrame(render);

    const output = solve(year, day, part, input);
    console.log('output: ' + output);
  }
}

run();

const currentYear = new URLSearchParams(window.location.search).get('year') || '2020';
document.querySelector('h1').textContent = `Advent of Code ${currentYear} timings`;

async function run() {
  const baselineResponse = await fetch("https://fornwall.net/baseline.json");
  const baselineJson = await baselineResponse.json();
  const benchmarks = Object.entries(baselineJson['benchmarks']);
  const extractMeanTime = (info) => info['criterion_estimates_v1']['mean']['point_estimate'];
  let totalRuntime = 0;

  const part_1s = {
    x: [],
    y: [],
    type: 'bar',
    name: 'Part 1'
  };
  const part_2s = {
    x: [],
    y: [],
    type: 'bar',
    name: 'Part 2'
  };
  for (const [benchmarkName, benchmarkInfo] of benchmarks) {
     if (!benchmarkName.startsWith(currentYear)) continue;
     const [year, day, part] = benchmarkName.split('_');
     const meanTime = extractMeanTime(benchmarkInfo);
     totalRuntime += meanTime;
     const runtimeMicroSeconds = Math.round(meanTime / 1000);
     const trace = (part == '1') ? part_1s : part_2s;
     trace.x.push(`Day ${day}`);
     trace.y.push(runtimeMicroSeconds);
  }
  const barLayout = {
      title: "Execution time per day and part",
      barmode: 'group',
      yaxis: {
        title: 'Execution time (microseconds)'
      }
  };
  Plotly.newPlot('firstPlot', [part_1s, part_2s], barLayout);

  const pieData = {
      values: [],
      labels: [],
      type: 'pie',
  };
  for (const [benchmarkName, benchmarkInfo] of benchmarks) {
     if (!benchmarkName.startsWith(currentYear)) continue;
     let [year, day, part] = benchmarkName.split('_');
     let percentageRuntime = 100. * extractMeanTime(benchmarkInfo) / totalRuntime;
     pieData.labels.push(`Day ${day}, Part ${part}`);
     pieData.values.push(percentageRuntime);
  }
  const pieLayout = {
    title: `Total: ${Number(Math.round(totalRuntime / 1000.)).toLocaleString()} microseconds`
  };
  Plotly.newPlot('secondPlot', [pieData], pieLayout);

  window.addEventListener("resize", () => {
    Plotly.Plots.resize(document.getElementById("firstPlot"));
    Plotly.Plots.resize(document.getElementById("secondPlot"));
  });
}
run();

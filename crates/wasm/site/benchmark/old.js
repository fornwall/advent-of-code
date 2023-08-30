const baselineJsonPromise = (async () => {
  const baselineResponse = await fetch("new-baseline.json");
  return await baselineResponse.json();
})();

async function updatePage() {
  const currentYear =
    new URLSearchParams(window.location.hash.substring(1)).get("year") ||
    "2020";

  document.querySelector(
    "h1",
  ).textContent = `Advent of Code ${currentYear} execution times`;

  const baselineJson = await baselineJsonPromise;
  const benchmarks = Object.entries(baselineJson["benchmarks"]);
  const extractMeanTime = (info) =>
    info["criterion_estimates_v1"]["mean"]["point_estimate"];
  let totalRuntime = 0;

  const part1s = {
    x: [],
    y: [],
    type: "bar",
    name: "Part 1",
  };
  const part2s = {
    x: [],
    y: [],
    type: "bar",
    name: "Part 2",
  };
  for (const [benchmarkName, benchmarkInfo] of benchmarks) {
    if (!benchmarkName.startsWith(currentYear)) continue;
    const [, day, part] = benchmarkName.split("_");
    const meanTime = extractMeanTime(benchmarkInfo);
    totalRuntime += meanTime;
    const runtimeMicroSeconds = Math.round(meanTime / 1000);
    const trace = part == "1" ? part1s : part2s;
    trace.x.push(`Day ${day}`);
    trace.y.push(runtimeMicroSeconds);
  }
  const barLayout = {
    // showlegend: false,
    barmode: "group",
    yaxis: {
      title: "Execution time (microseconds)",
    },
  };
  Plotly.newPlot("firstPlot", [part1s, part2s], barLayout);

  const pieData = {
    values: [],
    labels: [],
    type: "pie",
  };
  for (const [benchmarkName, benchmarkInfo] of benchmarks) {
    if (!benchmarkName.startsWith(currentYear)) continue;
    let [, day, part] = benchmarkName.split("_");
    let percentageRuntime =
      (100 * extractMeanTime(benchmarkInfo)) / totalRuntime;
    pieData.labels.push(`Day ${day}, Part ${part}`);
    pieData.values.push(percentageRuntime);
  }
  const pieLayout = {
    title: `Total: ${Number(
      Math.round(totalRuntime / 1000),
    ).toLocaleString()} microseconds`,
  };
  Plotly.newPlot("secondPlot", [pieData], pieLayout);

  window.addEventListener("resize", () => {
    Plotly.Plots.resize(document.getElementById("firstPlot"));
    Plotly.Plots.resize(document.getElementById("secondPlot"));
  });
}

updatePage();

window.addEventListener("hashchange", updatePage);

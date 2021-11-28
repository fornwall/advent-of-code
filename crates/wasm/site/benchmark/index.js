const baselineJsonPromise = (async () => {
  const baselineResponse = await fetch("new-baseline.json");
  return await baselineResponse.json();
})();

function sumDown(map) {
  let result = 0;
  for (const entry of Object.values(map)) {
    if (typeof entry == "object") {
      result += sumDown(entry);
    } else {
      result += entry;
    }
  }
  return result;
}

async function updatePage() {
  const baselineJson = await baselineJsonPromise;
  const benchmarks = Object.entries(baselineJson["benchmarks"]);
  const extractMeanTime = (info) =>
    info["criterion_estimates_v1"]["mean"]["point_estimate"];

  const dataMap = {};
  for (const [benchmarkName, benchmarkInfo] of benchmarks) {
    const [year, dayString, part] = benchmarkName.split("_");
    const day = parseInt(dayString, 10);
    const meanTime = extractMeanTime(benchmarkInfo) / 1000_000;
    if (!dataMap[year]) dataMap[year] = {};
    if (!dataMap[year][day]) dataMap[year][day] = {};
    dataMap[year][day][part] = meanTime;
  }

  const data = {
    //maxdepth: 3,
    type: "sunburst",
    labels: ["All years"],
    parents: [""],
    values: [sumDown(dataMap)],
    outsidetextfont: { size: 20, color: "#377eb8" },
    //marker: {line: {width: 2}},
    branchvalues: "total",
  };

  for (const [year, yearData] of Object.entries(dataMap).sort(
    (a, b) => a[0] < b[0]
  )) {
    const yearString = `${year}`;
    data.labels.push(yearString);
    data.parents.push("All years");
    data.values.push(sumDown(yearData));
    for (let i = 1; i <= 25; i++) {
      const dayLabel = `${year} day ${i}`;
      data.labels.push(dayLabel);
      data.parents.push(yearString);
      data.values.push(sumDown(yearData[i]));

      for (let part = 1; part <= 2; part++) {
        if (yearData[i][part]) {
          data.labels.push(`${year} day ${i} part ${part}`);
          data.parents.push(dayLabel);
          data.values.push(yearData[i][part]);
        }
      }
    }
  }
  console.log(data);

  const layout = {
    margin: { l: 0, r: 0, b: 0, t: 0 },
    //extendsunburstcolorway: true
  };

  Plotly.newPlot("firstPlot", [data], layout, { displaylogo: false });
  window.addEventListener("resize", () => {
    Plotly.Plots.resize(document.getElementById("firstPlot"));
  });
}

updatePage();

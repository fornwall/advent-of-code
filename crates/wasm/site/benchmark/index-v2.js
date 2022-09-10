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
  const benchmarks = Object.entries(baselineJson);

  const dataMap = {};
  for (const [_benchmarkName, benchmarkInfo] of benchmarks) {
    const year = benchmarkInfo["year"];
    const day = benchmarkInfo["day"];
    const part = benchmarkInfo["part"];
    const cycles = benchmarkInfo["cycles"];
    if (!dataMap[year]) dataMap[year] = {};
    if (!dataMap[year][day]) dataMap[year][day] = {};
    dataMap[year][day][part] = cycles;
  }

  const data = {
    type: "sunburst",
    labels: ["All years"],
    parents: [""],
    values: [sumDown(dataMap)],
    outsidetextfont: { size: 20, color: "#377eb8" },
    branchvalues: "total",
    sort: false,
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

  const layout = {
    margin: { l: 0, r: 0, b: 0, t: 0 },
  };

  Plotly.newPlot("firstPlot", [data], layout, {
    displaylogo: false,
    responsive: true,
    scrollZoom: true,
    toImageButtonOptions: {
      format: "svg",
    },
  });
}

updatePage();

import gistMapping from "../gist-mapping.json";

async function main() {
  addEventListener("error", (e) => {
    console.log("error", e);
    alert(e.message);
  });

  document.getElementById("rerun-button").addEventListener("click", (e) => {
    window.location.reload();
  });

  const year = parseInt(
    new URLSearchParams(window.location.search).get("year") || "2025",
  );
  document.getElementById("description").innerHTML = `Benchmark running
          <a href="/">solutions</a> to
          <a href="https://adventofcode.com/${year}/">Advent of Code ${year}</a> in
          the browser using WebAssembly.`;

  const yearSelect = document.getElementById("year");
  yearSelect.value = year;
  yearSelect.addEventListener("change", () => {
    document.getElementById("form").submit();
  });

  const tests = await (
    await fetch("https://fornwall.net/aoc/tests.json")
  ).json();
  const yearDays = tests.years.find((y) => y.year == year).days;
  const lastDay = year < 2025 ? 25 : 12;
  const tbody = document.querySelector("tbody");

  const worker = new Worker(new URL("../worker-wasm.js", import.meta.url), {
    name: "wasm-runner",
    type: "module",
  });

  worker.onmessage = (e) => {
    if (!e.data.wasmWorking) {
      alert("WASM not working");
      return;
    }
    // Ignore initial warmup response:
    let initialMessageCount = 0;
    const times = [];
    worker.onmessage = (e) => {
      initialMessageCount++;
      if (initialMessageCount > 1) {
        times.push(e.data);

        const expectedAnswer = yearDays.find((d) => d.day == e.data.day)[
          "part" + e.data.part
        ];
        if (expectedAnswer != e.data.output) {
          const message = `Error for ${year}-${e.data.day}-${e.data.part}: Expected "${expectedAnswer}, was "${e.data.output}"`;
          console.error(message);
          alert(message);
        }

        if (e.data.day == lastDay) {
          const totalTime = times
            .map((d) => d.executionTime)
            .reduce((a, b) => a + b, 0);
          document.getElementById("total-time").textContent =
            `Total time: ${totalTime.toLocaleString(undefined, {
              minimumFractionDigits: 2,
            })} ms`;
          for (const data of times) {
            const tr = document.createElement("tr");
            const percentageTime = (data.executionTime * 100) / totalTime;

            let problemLink = `https://adventofcode.com/${data.year}/day/${data.day}`;
            if (data.part == 2) problemLink += "#part2";

            const gistLink =
              "https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=" +
              gistMapping[data.year][data.day]["gist"];
            const compilerExplorerLink =
              "https://godbolt.org/z/" +
              gistMapping[data.year][data.day]["compiler_explorer"];

            tr.innerHTML = `<td class="text-end"><a href="${problemLink}">${
              data.day
            }-${
              data.part
            }</a></td><td class="text-end">${data.executionTime.toFixed(
              2,
            )}</td><td class="text-end">${percentageTime.toFixed(2)}</td>
          <td><a href="${gistLink}">src</a></td>
          <td><a href="${compilerExplorerLink}">asm</a></td>`;
            tbody.appendChild(tr);
          }

          const yearLabel = `${year}`;
          const data = {
            type: "sunburst",
            labels: [yearLabel],
            parents: [""],
            values: [totalTime],
            outsidetextfont: { size: 20, color: "#fff" },
            branchvalues: "total",
            sort: false,
          };

          for (let day = 1; day <= lastDay; day++) {
            const dayLabel = `Day ${day}`;
            const dayTime = times
              .filter((d) => d.day == day)
              .map((d) => d.executionTime)
              .reduce((a, b) => a + b, 0);
            data.labels.push(dayLabel);
            data.parents.push(yearLabel);
            data.values.push(dayTime);
            for (let part = 1; part <= 2; part++) {
              if (day === lastDay && part === 2) continue;
              const partTime = times.filter(
                (d) => d.day == day && d.part == part,
              )[0].executionTime;
              data.labels.push(`Day ${day} part ${part}`);
              data.parents.push(dayLabel);
              data.values.push(partTime);
            }
          }

          const layout = {
            margin: { l: 0, r: 0, b: 0, t: 0 },
            paper_bgcolor: "rgb(15,15,35)",
          };

          Plotly.newPlot("plot", [data], layout, {
            displayModeBar: false,
            displaylogo: false,
            responsive: true,
            scrollZoom: true,
          });

          document.getElementById("spinner").remove();
          document.getElementById("result").classList.remove("invisible");
        }
      }
    };

    for (const day of yearDays) {
      const input = day.input;
      for (let part = 1; part < 3; part++) {
        if (!(day.day == lastDay && part == 2)) {
          worker.postMessage({ year, day: day.day, part, input });
          if (day.day == 1 && part == 1) {
            worker.postMessage({ year, day: day.day, part, input });
            // Again - initial was warmup
          }
        }
      }
    }
  };
}

main();

"use strict";
async function solveApi(host, year, day, part, input) {
  const startTime = performance.now();
  try {
    const response = await fetch(
      `https://${host}/solve/${year}/${day}/${part}`,
      {
        method: "POST",
        headers: { "content-type": "text/plain" },
        body: input,
      }
    );

    if (![200, 400].includes(response.status)) {
      throw new Error(`Unexpected response status code: ${response.status}`);
    }

    const responseText = await response.text();
    const executionTime = performance.now() - startTime;
    console.log(
      `API ${year}-${day}-${part} response from ${host}: ${executionTime.toFixed(
        2
      )} ms`
    );
    return {
      answer: responseText,
      isError: !response.ok,
      executionTime,
    };
  } catch (e) {
    throw new Error(`https://${host}: ${e.message}`);
  }
}

self.onmessage = async (message) => {
  const { year, day, part, input } = message.data;

  const apiPromise1 = solveApi("advent.fly.dev", year, day, part, input);
  const apiPromise2 = solveApi(
    "aoc.fornwall.workers.dev",
    year,
    day,
    part,
    input
  );
  const apiPromise3 = solveApi(
    "mystifying-blackwell-9e705f.netlify.app",
    year,
    day,
    part,
    input
  );

  Promise.any([apiPromise1, apiPromise2, apiPromise3])
    .then((response) => {
      postMessage({
        worker: "api",
        year,
        day,
        part,
        input,
        output: response.answer,
        isError: response.isError,
        executionTime: response.executionTime,
      });
    })
    .catch((e) => {
      const message = e.errors.map((e) => `• ${e.message}`).join("\n");
      postMessage({
        worker: "api",
        year,
        day,
        part,
        input,
        output: message,
        isError: true,
        isInternalError: true,
        executionTime: 0,
      });
    });
};

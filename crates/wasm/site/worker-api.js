const API_HOSTS = [
  "advent.fly.dev",
  "aoc.fornwall.workers.dev",
  "mystifying-blackwell-9e705f.netlify.app",
];

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

  const apiPromises = API_HOSTS.map((host) =>
    solveApi(host, year, day, part, input)
  );

  try {
    const response = await Promise.any(apiPromises);
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
  } catch (e) {
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
  }
};

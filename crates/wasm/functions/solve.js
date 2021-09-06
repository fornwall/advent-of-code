const solve = require("advent-of-code-rs-wasm").solve;

exports.handler = async function (event, context) {
  const headers = {
    "Access-Control-Allow-Origin": "*",
  };

  if (event.httpMethod !== "POST") {
    return {
      statusCode: 400,
      headers: {
        Allow: "POST",
      },
      body: "This endpoint only supports HTTP POST.",
    };
  }

  const pathParts = event.path.substring(1).split("/");
  if (pathParts.length != 4) {
    return {
      statusCode: 400,
      headers,
      body: "Invalid path - expected /solve/{YEAR}/{DAY}/{PART}, was: " + path,
    };
  }
  const [, year, day, part] = pathParts;

  try {
    const input = event.body;
    const solution = solve(year, day, part, input);
    return {
      statusCode: 200,
      headers,
      body: solution,
    };
  } catch (e) {
    return {
      statusCode: 400,
      headers,
      body: e.message,
    };
  }
};

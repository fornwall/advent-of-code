const solve = require('advent-of-code-rs-wasm').solve;

exports.handler = function(event, context, callback) {
  if (event.httpMethod !== 'POST') {
    return callback(null, {
      statusCode: 400,
      body: 'This endpoint only supports HTTP POST.',
      headers: {
        'Allow': 'POST'
      }
    });
  }

  const pathParts = event.path.substring(1).split('/');
  if (pathParts.length != 4) {
    return callback(null, {
      statusCode: 200,
      body: "Invalid path - expected /api/{YEAR}/{DAY}/{PART}, was: " + path
    });
  }
  const [_api, year, day, part] = pathParts;

  try {
    const input = event.body;
    const solution = solve(year, day, part, input);
    return callback(null, {
      statusCode: 200,
      body: solution
    });
  } catch (e) {
    return callback(null, {
      statusCode: 400,
      body: e.message
    });
  }
}

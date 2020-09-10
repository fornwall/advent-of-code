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

  const parameters = event.queryStringParameters;
  const year = parseInt(parameters.year || '0');
  const day = parseInt(parameters.day || '1');
  const part = parseInt(parameters.part || '1')

  if (!(year >= 2018 && year <= 2019)) {
    return callback(null, {
      statusCode: 400,
      body: 'Invalid year - must be integer between 2018 and 2019'
    });
  } else if (!(day >= 1 && day <= 25)) {
    return callback(null, {
      statusCode: 400,
      body: 'Invalid day - must be integer between 1 and 25'
    });
  } else if (!(part >= 1 && part <= 2)) {
    return callback(null, {
      statusCode: 400,
      body: 'Invalid part - must be 1 or 2'
    });
  }


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
      body: "Invalid input"
    });
  }
}

const solve = require('advent_of_code_rs').solve;

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
  const day = parseInt(parameters.day || '1');
  const part = parseInt(parameters.part || '1')

  if (!(day >= 1 && day <= 25)) {
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

  const input = event.body;
  const solution = solve(day, part, input);

  return callback(null, {
    statusCode: 200,
    body: solution
  });
}

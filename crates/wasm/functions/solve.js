const solve = require('advent_of_code_rs').solve;

exports.handler = function(event, context, callback) {
    const parameters = event.queryStringParameters;
    const day = parseInt(parameters.day || '1');
    const part = parseInt(parameters.part || '1')

	console.log('About to log body');
	console.log(event.body);

    let input = '12';
    const solution = solve(day, part, input);

    callback(null, {
        statusCode: 200,
        body: "day=" + day + ", part=" + part + ", Solution: " + solution
    });
}

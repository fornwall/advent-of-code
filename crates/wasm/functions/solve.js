const solve = require('advent_of_code_rs').solve;

exports.handler = function(event, context, callback) {

    let day = 1;
    let part = 1;
    let input = '12';
    const solution = solve(day, part, input);

    callback(null, {
        statusCode: 200,
        body: "Solution: " + solution
    });
}

addEventListener('fetch', (event) => {
  event.respondWith(handleRequest(event.request))
})


function badInput(errorMessage) {
  console.error(errorMessage);
  return new Response(errorMessage, {status: 400});
}


async function handleRequest(request) {
  var path = new URL(request.url).pathname;
  var pathParts = path.substring(1).split('/');
  if (pathParts.length != 3) {
    return badInput("Invalid path - expected /{YEAR}/{DAY}/{PART}, was: " + path);
  }
  const [year, day, part] = pathParts;
  const input = await request.text();

  const { solve } = wasm_bindgen;
  await wasm_bindgen(wasm);

  try {
    const solution = solve(year, day, part, input);
    const response = new Response(solution, {status: 200});
    response.headers.set("Access-Control-Allow-Origin", "*");
    return response
  } catch (e) {
    return badInput(e.message);
  }
}

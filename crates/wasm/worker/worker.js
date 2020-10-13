addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { searchParams } = new URL(request.url);
    let year = searchParams.get("year");
    let day = searchParams.get("day");
    let part = searchParams.get("part");
    let input = await request.text();

    const { solve } = wasm_bindgen;
    await wasm_bindgen(wasm);
    try {
        const solution = solve(year, day, part, input);
        return new Response(solution, {status: 200});
    } catch (e) {
        console.error(e.message);
        return new Response(e.message, {status: 400});
    }
}

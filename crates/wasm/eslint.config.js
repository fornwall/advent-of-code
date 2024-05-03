export default [
  { files: ["**/*.js"] },
  {
    ignores: [
      "**/*min.js",
      "**/advent_of_code_wasm.js",
      "**/dist",
      "**/generated",
      "**/target",
      "/aoc.fornwall.net/",
      "site/static/boostrap.min.css*",
    ],
  },
];

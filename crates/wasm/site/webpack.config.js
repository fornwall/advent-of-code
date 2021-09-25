module.exports = {
  entry: "./index.js",
  output: {
    path: __dirname,
    publicPath: "",
    filename: "index.min.js",
  },
  devServer: {
    static: {
      directory: __dirname,
    },
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
  },
};

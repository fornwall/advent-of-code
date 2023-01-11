const HtmlWebpackPlugin = require("html-webpack-plugin");
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: {
      "home": "./index.js",
      "runbench": "./runbench/index.js",
      "show": "./show/index.js",
  },
  output: {
    publicPath: "/",
    filename: function(pathData) {
        if (pathData.chunk.name === 'visualizer') {
            return 'show/[name].[contenthash].js';
        }
        return "[name]/[name].[contenthash].js";
    }
  },
  devServer: {
    static: __dirname,
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
  },
  plugins: [
    new HtmlWebpackPlugin({
      filename: "index.html",
      template: "index.html",
      chunks: ["home"],
      metadata: {
        baseUrl: "/",
      }
    }),
    new HtmlWebpackPlugin({
      filename: "show/index.html",
      template: "show/index.html",
      chunks: ["show"],
      publicPath: "/",
      metadata: {
        baseUrl: "/show/",
      }
    }),
    new HtmlWebpackPlugin({
      filename: "runbench/index.html",
      template: "runbench/index.html",
      chunks: ["runbench"],
    }),
    new CopyWebpackPlugin({
        patterns: [ { from: 'static', to: 'static' } ],
    })
  ],
};

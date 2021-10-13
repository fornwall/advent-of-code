const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: "./index.js",
  output: {
    publicPath: "",
    filename: "[name].[contenthash].js"
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
  plugins: [
    new HtmlWebpackPlugin({
        template: 'index.html',
    })
  ]
};

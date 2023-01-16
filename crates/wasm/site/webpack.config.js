import HtmlWebpackPlugin from "html-webpack-plugin";
import CopyWebpackPlugin from "copy-webpack-plugin";

export default {
  entry: {
    home: "./index.js",
    runbench: "./runbench/index.js",
    show: "./show/index.js",
  },
  output: {
    publicPath: "/",
    assetModuleFilename: "asset-[name]-[contenthash][ext]",
    filename: "bundle-[name]-[contenthash].js",
    chunkFilename: "chunk-[name]-[contenthash].js",
  },
  devServer: {
    static: ".",
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
      },
    }),
    new HtmlWebpackPlugin({
      filename: "show/index.html",
      template: "show/index.html",
      chunks: ["show"],
      publicPath: "/",
      metadata: {
        baseUrl: "/show/",
      },
    }),
    new HtmlWebpackPlugin({
      filename: "runbench/index.html",
      template: "runbench/index.html",
      chunks: ["runbench"],
    }),
    new CopyWebpackPlugin({
      patterns: [{ from: "static", to: "static" }],
    }),
  ],
};

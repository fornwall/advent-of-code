import HtmlWebpackPlugin from "html-webpack-plugin";
import CopyWebpackPlugin from "copy-webpack-plugin";
import WorkboxPlugin from "workbox-webpack-plugin";

export default {
  entry: {
    home: "./index.js",
    runbench: "./runbench/index.js",
    show: "./show/index.js",
    code: "./code/index.js",
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
    }),
    new HtmlWebpackPlugin({
      filename: "show/index.html",
      template: "show/index.html",
      chunks: ["show"],
    }),
    new HtmlWebpackPlugin({
      filename: "runbench/index.html",
      template: "runbench/index.html",
      chunks: ["runbench"],
    }),
    new HtmlWebpackPlugin({
      filename: "code/index.html",
      template: "code/index.html",
      chunks: ["code"],
    }),
    new CopyWebpackPlugin({
      patterns: [{ from: "static", to: "static" }],
    }),
    // See https://webpack.js.org/guides/progressive-web-application/
    new WorkboxPlugin.GenerateSW({
      // these options encourage the ServiceWorkers to get in there fast
      // and not allow any straggling "old" SWs to hang around
      clientsClaim: true,
      skipWaiting: true,
    }),
  ],
};

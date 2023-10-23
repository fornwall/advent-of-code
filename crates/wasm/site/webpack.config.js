import HtmlWebpackPlugin from "html-webpack-plugin";
import CopyWebpackPlugin from "copy-webpack-plugin";
import WorkboxPlugin from "workbox-webpack-plugin";

const plugins = [
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
];

if (process.env.NODE_ENV === "production") {
  // See https://webpack.js.org/guides/progressive-web-application/
  plugins.push(
    new WorkboxPlugin.GenerateSW({
      // these options encourage the ServiceWorkers to get in there fast
      // and not allow any straggling "old" SWs to hang around
      cleanupOutdatedCaches: true,
      clientsClaim: true,
      skipWaiting: true,
      directoryIndex: "index.html",
    }),
  );
}

export default {
  entry: {
    home: "./index.js",
    runbench: "./runbench/index.js",
    show: "./show/index.js",
    code: "./code/index.js",
  },
  output: {
    assetModuleFilename: "asset-[name]-[contenthash][ext]",
    filename: "bundle-[name]-[contenthash].js",
    chunkFilename: "chunk-[name]-[contenthash].js",
  },
  devServer: {
    static: ".",
    allowedHosts: "all",
    headers: {
      "Cross-Origin-Embedder-Policy": "require-corp",
      "Cross-Origin-Opener-Policy": "same-origin",
    },
    // https://webpack.js.org/configuration/dev-server/#websocketurl:
    // To get protocol/hostname/port from browser use:
    // client: { webSocketURL: "auto://0.0.0.0:0/laptop/ws" },
  },
  plugins,
};

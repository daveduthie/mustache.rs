const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "development",
  entry: {
    bootstrap: "./js/bootstrap.js",
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  devServer: {
    static: dist,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: "static", to: "." },
        { from: "fib/pkg/*.wasm", to: "." },
      ],
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "mustache"),
      extraArgs: "-- --features console_error_panic_hook",
      // forceMode: "production",
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};

const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: "development",
  entry: {
    main: "./target/public/cljs-out/dev/main.js",
    base: "./target/public/cljs-out/dev/cljs_base.js",
    big: "./target/public/cljs-out/dev/big.js",
    index: "./src/js/index.js",
  },
  resolve: {
    alias: {
      libhunam: path.resolve(__dirname, "src/ts/libhunam.tsx"),
    },
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.css$/i,
        use: ["style-loader", "css-loader"],
      },
    ],
  },
  output: {
    filename: "[name].js",
  },
  devServer: {
    static: ["./target/public", "./resources/public"],
    liveReload: false,
    hot: true,
    compress: true,
    port: 9001,
  },
  plugins: [
    new CopyPlugin({
      patterns: [{ from: "./target/public/cljs-out/dev", to: "." }],
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../mustache"),
      extraArgs: "-- --features console_error_panic_hook",
    }),
  ],
  devtool: "inline-source-map",
  experiments: {
    asyncWebAssembly: true,
    // futureDefaults: true,
  },
};

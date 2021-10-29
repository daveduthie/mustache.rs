const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  mode: "development",
  devServer: {
    static: ["./target/public", "./resources/public"],
    compress: true,
    port: 9000,
  },
  plugins: [
    new CopyPlugin({
      patterns: [{ from: "./target/public/cljs-out/dev", to: "." }],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
    futureDefaults: true,
  },
};

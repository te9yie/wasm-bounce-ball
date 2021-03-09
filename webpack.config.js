const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  mode: "development",
  entry: "./pkg/index.js",
  plugins: [
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  devServer: {
    port: 5000,
    open: true,
  },
  experiments: {
    asyncWebAssembly: true,
  },
};

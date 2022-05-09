const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin([
      "index.html",
      "style.css",
      "worker.js",
      { from: "pkg/tsumeshogi_solver_wasm.js", to: "pkg" },
      { from: "pkg/tsumeshogi_solver_wasm_bg.wasm", to: "pkg" },
    ]),
  ],
};

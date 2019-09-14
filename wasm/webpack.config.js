const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

const src = __dirname;
const dist = path.resolve(__dirname, "/dist");

module.exports = {
  context: src,
  entry: "./index.js",
  mode: "production",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js"
  },
  devServer: {
    contentBase: dist,
    port: 4444
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "./index.html"
    })
  ]  
};

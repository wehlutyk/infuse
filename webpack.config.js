const path = require('path');

module.exports = {
  context: path.resolve(__dirname, "app"),
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development"
};

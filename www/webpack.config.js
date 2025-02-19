const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async",
      },
      {
        test: /\.css$/i,
        use: [
          MiniCssExtractPlugin.loader, // Extract CSS to files
          'css-loader', // Translates CSS into CommonJS
          'postcss-loader', // Processes CSS with PostCSS (Tailwind)
        ],
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: 'index.html', to: 'index.html' },
        { from: 'favicon.ico', to: 'favicon.ico' },
      ],
    }),
    new MiniCssExtractPlugin({
      filename: 'style.css', // Output CSS filename
    }),
  ],
};

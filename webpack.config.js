'use strict';

const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
  mode: 'production',
  devtool: 'source-map',
  entry: {
    app: './assets/typescript/index.ts'
  },
  output: {
    path: `${__dirname}/public/`,
    filename: 'js/main.js'
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: { loader: 'ts-loader' }
      },
      {
        test: /\.scss$/,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader',
          'sass-loader',
        ]
      },
    ]
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "css/main.css"
    })
  ],
  resolve: { extensions: ['.ts'] }
};

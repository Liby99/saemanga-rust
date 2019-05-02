'use strict';

const MiniCssExtractPlugin = require("mini-css-extract-plugin");

module.exports = {
  mode: 'development',
  devtool: 'source-map',
  entry: {
    app: './assets/typescript/main.ts',
  },
  output: {
    path: `${__dirname}/public/`,
    filename: 'js/main.js'
  },
  module: {
    rules: [{
      test: /\.ts$/,
      exclude: /node_modules/,
      use: {
        loader: 'ts-loader'
      }
    }, {
      test: /\.scss$/,
      use: [
        MiniCssExtractPlugin.loader,
        'css-loader',
        'sass-loader',
      ]
    }, {
      test: /images\/.+\.(svg|png|jpg|jpeg|gif)/,
      use: [{
        loader: "file-loader",
        options: {
          name: 'img/[name].[ext]'
        }
      }]
    }, {
      test: /\.(svg|woff|woff2|ttf|eot)$/,
      exclude: /images/,
      use: [{
        loader: "file-loader",
        options: {
          publicPath: '../',
          name: 'fonts/[name].[ext]'
        }
      }]
    }, {
      test: /\.html\.hbs$/,
      use: [{
        loader: "handlebars-loader",
        options: {
          rootRelative: ""
        }
      }],
    }]
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "css/main.css"
    })
  ],
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.json']
  },
  performance: {
    hints: false
  }
};

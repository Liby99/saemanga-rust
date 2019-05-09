'use strict';

const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');

module.exports = {
  mode: 'development',
  devtool: 'source-map',
  cache: true,
  entry: {
    'index': './assets/typescript/index.ts',
    'admin': './assets/typescript/admin.ts',
  },
  output: {
    path: `${__dirname}/public/`,
    filename: 'js/[name].js'
  },
  module: {
    rules: [{
      test: /\.ts$/,
      exclude: /node_modules/,
      use: {
        loader: 'ts-loader',
        options: {
          transpileOnly: true
        }
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
      filename: "css/[name].css"
    }),
    new ForkTsCheckerWebpackPlugin(),
  ],
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.json']
  },
  performance: {
    hints: false
  }
};

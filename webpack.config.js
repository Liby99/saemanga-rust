'use strict';

module.exports = {
  mode: 'production',
  devtool: 'source-map',
  entry: {
    app :'./assets/typescript/index.ts'
  },
  output: {
    path: `${__dirname}/public/js`,
    filename: 'main.js'
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: {
          loader: 'ts-loader'
        }
      }
    ]
  },
  resolve: { extensions: ['.ts'] }
};

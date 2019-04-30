'use strict';

const gulp = require('gulp');
const webpackStream = require('webpack-stream');
const webpackConfig = require('./webpack.config.js');
const webpack = require('webpack');
const del = require('del');

const TS_SRC = './assets/typescript/**/*.ts';
const JS_DEST = './public/js/*';

gulp.task('clean', function() {
  del([JS_DEST]);
});

gulp.task('webpack', function() {
  return webpackStream(webpackConfig, webpack)
    .pipe(gulp.dest("public/js/"));
});

gulp.task('watch', function () {
  gulp.watch(TS_SRC, ['webpack']);
});

gulp.task('default', ['webpack']);
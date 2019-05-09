// `Colors` doesn't need a local variable
require('colors');

// Imports
const gulp = require('gulp');
const nodemon = require('gulp-nodemon');
const watch = require('gulp-watch');
const buildTask = require('./scripts/gulp_build_task');

gulp.task("build-cargo", buildTask('cargo', 'cargo build --color always'));

gulp.task("build-webpack", buildTask('webpack', 'npx webpack --colors'));

gulp.task('build', gulp.series(
  "build-cargo",
  "build-webpack"
));

gulp.task('dev-run', (() => {
  let nodemonInstance = undefined;
  return (done) => {
    if (nodemonInstance) {
      nodemonInstance.emit('restart');
    } else {
      nodemonInstance = nodemon({
        done: done,
        watch: ["--non-existing-folder--"],
        exec: "cargo run",
      });
    }
  }
})());

gulp.task('dev-watch', (done) => {
  watch(['src/', 'assets/'], {
    read: false,
  }, (file) => {
    const isRustChange = file.extname === '.rs';
    const task = isRustChange ? "build-cargo" : "build-webpack";
    // console.log(`${"[dev-watch]".cyan} Detected ${isRustChange ? "back-end" : "front-end"} changes. Rebuilding.`);
    gulp.task(task)((err) => {
      if (err) {
        console.log(`${"[dev-watch]".red} Error detected. Waiting for changes...`);
      } else {
        gulp.task('dev-run')(done);
      }
    });
  });
});

gulp.task('dev', (done) => {
  gulp.task('build')((err) => {
    if (err) console.log(`${"[dev]".red} Error detected. Waiting for changes...`);
    else gulp.task('dev-run')(done);
    gulp.task('dev-watch')(done);
  });
});
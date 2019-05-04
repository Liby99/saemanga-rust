const gulp = require('gulp');
const nodemon = require('gulp-nodemon');
const watch = require('gulp-watch');
const { exec } = require('child_process');

let nodemonInstance;

const events = {};

function opened(evt) {
  return events[evt] !== undefined;
}

function open(evt) {
  events[evt] = [];
}

function wait(evt, fn) {
  if (opened(evt)) events[evt].push(fn);
}

function emit(evt) {
  for (const fn of events[evt]) fn();
  delete events[evt];
}

gulp.task("build-cargo", (done) => {
  if (!opened('cargo')) {
    open('cargo');
    process.stdout.write("Building cargo... ");
    exec("cargo build --color always", (err, stdout, stderr) => {
      if (err) {
        console.log("Failed");
        console.log(stdout);
        console.error(stderr);
      } else {
        console.log("Done");
      }
      emit('cargo');
      done(err);
    });
  } else {
    wait('cargo', done);
  }
});

gulp.task("build-webpack", (done) => {
  if (!opened('webpack')) {
    open('webpack')
    process.stdout.write("Building webpack... ");
    exec("npx webpack --colors", (err, stdout, stderr) => {
      if (err) {
        console.log("Failed");
        console.log(stdout);
        console.error(stderr);
      } else {
        console.log("Done");
      }
      emit('webpack');
      done(err);
    });
  } else {
    wait('webpack', done);
  }
});

gulp.task('build', gulp.series(
  "build-cargo",
  "build-webpack"
));

gulp.task('dev-run', (done) => {
  if (nodemonInstance) {
    nodemonInstance.emit('restart');
  } else {
    nodemonInstance = nodemon({
      done: done,
      watch: ["--non-existing-folder--"],
      exec: "cargo run",
    });
  }
});

gulp.task('dev-watch', (done) => {
  return watch(['src/', 'assets/'], {
    read: false,
  }, (file) => {
    const isRustChange = file.extname === '.rs';
    const task = isRustChange ? "build-cargo" : "build-webpack";
    console.log(`Detected ${isRustChange ? "back-end" : "front-end"} changes. Rebuilding.`);
    gulp.task(task)((err) => {
      if (err) {
        console.log("Error detected. Waiting for changes...");
        gulp.task('dev-watch')(done);
      } else {
        gulp.task('dev-run')(done);
      }
    });
  });
});

gulp.task('dev', (done) => {
  gulp.task('build')((err) => {
    if (err) console.log("Error detected. Waiting for changes...");
    else gulp.task('dev-run')(done);
    gulp.task('dev-watch')(done);
  });
});
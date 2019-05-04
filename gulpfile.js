// `Colors` doesn't need a local variable
require('colors');

// Imports
const gulp = require('gulp');
const nodemon = require('gulp-nodemon');
const watch = require('gulp-watch');
const { exec } = require('child_process');

// Need to keep track of the global nodemon instance
let nodemonInstance;

// Event handling
class EventPool {
  constructor() {
    this.events = {};
  }

  opened(evt) {
    return this.events[evt] !== undefined;
  }

  open(evt) {
    this.events[evt] = [];
  }

  wait(evt, fn) {
    if (this.opened(evt)) this.events[evt].push(fn);
  }

  emit(evt) {
    for (const fn of this.events[evt]) fn();
    delete this.events[evt];
  }
}

const pool = new EventPool();

gulp.task("build-cargo", (done) => {
  if (!pool.opened('cargo')) {
    pool.open('cargo');
    process.stdout.write(`${"[build]".cyan} Building cargo... `);
    exec("cargo build --color always", (err, stdout, stderr) => {
      if (err) {
        console.log("Failed".red);
        console.log(stdout);
        console.error(stderr);
      } else {
        console.log("Done".green);
      }
      pool.emit('cargo');
      done(err);
    });
  } else {
    pool.wait('cargo', done);
  }
});

gulp.task("build-webpack", (done) => {
  if (!pool.opened('webpack')) {
    pool.open('webpack');
    process.stdout.write(`${"[build]".cyan} Building webpack... `);
    exec("npx webpack --colors", (err, stdout, stderr) => {
      if (err) {
        console.log("Failed".red);
        console.log(stdout);
        console.error(stderr);
      } else {
        console.log("Done".green);
      }
      pool.emit('webpack');
      done(err);
    });
  } else {
    pool.wait('webpack', done);
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
  watch(['src/', 'assets/'], {
    read: false,
  }, (file) => {
    const isRustChange = file.extname === '.rs';
    const task = isRustChange ? "build-cargo" : "build-webpack";
    console.log(`${"[dev-watch]".cyan} Detected ${isRustChange ? "back-end" : "front-end"} changes. Rebuilding.`);
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
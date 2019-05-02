const gulp = require('gulp');
const nodemon = require('gulp-nodemon');
const exec = require('child_process').exec;
const path = require('path');

const backendFileExt = [".rs"];
const frontendFileExt = [".scss", ".js", ".json", ".ts", ".hbs"];
const backendBuildTask = "build-back-end";
const frontendBuildTask = "build-front-end";

let nodemonInstance;

gulp.task(backendBuildTask, (done) => {
  exec("cargo build", (err, stdout, stderr) => {
    console.log(stdout);
    console.error(stderr);
    done(err);
  });
});

gulp.task(frontendBuildTask, (done) => {
  exec("npx webpack", (err, stdout, stderr) => {
    console.log(stdout);
    console.error(stderr);
    done(err);
  });
});

gulp.task('dev', (done) => {
  nodemonInstance = nodemon({
    watch: ["src/", "assets/"],
    ext: "js json ts rs hbs scss",
    verbose: true,
    exec: "cargo run",
    done: done,
    tasks: (changedFiles) => {

      // If no changed files then no task to do
      if (!changedFiles) return [];

      // Check back end or front file and push tasks
      return Object.keys(changedFiles.reduce((acc, file) => {
        const ext = path.extname(file);
        if (~backendFileExt.indexOf(ext)) {
          return { ...acc, [backendBuildTask]: true };
        } else if (~frontendFileExt.indexOf(ext)) {
          return { ...acc, [frontendBuildTask]: true };
        }
      }, {}));
    }
  }).on('restart', function () {
    console.log('restarted!')
  }).on('crash', function() {
    console.error('Application has crashed!\n');
  });
});
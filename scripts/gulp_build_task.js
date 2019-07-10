// Exec used for subprocess execution
const { exec } = require('child_process');

// Event pool used for event emission
const EventPool = require('./gulp_event_pool');

/**
 * A Gulp Build Task that is able to detect re-invoke of the same process
 * and always use the later one for execution. All the prior subprocess will
 * asynchronously return only when the last subprocess succeeded or failed.
 *
 * @param name [string] the name of this build task
 * @param cmd [string] the command to execute
 *
 * @return A build function that can be passed to gulp
 *
 * # Examples
 *
 * ```
 * gulp.task('build-webpack', buildTask('webpack', 'npx webpack --colors'))
 * ```
 */
module.exports = (name, cmd) => {

  // Closure variables
  const pool = new EventPool();
  let child_proc = undefined;

  // Return the task function
  return (done) => {

    // If child process already exists, kill it with sigint
    if (child_proc) {
      child_proc.kill("SIGINT");
      child_proc = undefined;
    }

    // If event pool isn't presented, then open it.
    if (!pool.opened()) {
      pool.open();
    }

    // Execute the child process with cmd
    child_proc = exec(cmd, (err, stdout, stderr) => {
      if (err) {
        if (err.killed) {
          console.log("Interrupted".magenta);
          pool.wait(done);
        } else {
          console.log("Failed".red);
          console.log(stdout);
          console.error(stderr);
          pool.emit();
          done(err);
        }
      } else {
        console.log("Done".green);
        pool.emit();
        child_proc = undefined;
        done();
      }
    });

    // Debug print. Delay the print because we first want to print "interrupted"
    setTimeout(() => {
      process.stdout.write(`${"[build]".cyan} Building ${name}... `);
    }, 0);
  }
}
# 冴えない漫画の育て方

My manga website finally comes with a rust implementation. This is not in production yet but will be soon integrated. So, I'm happy to announce that this is the 4.0 version of saemanga.com!

# Build

You need to first build the front-end assets. That will generate a directory called `/public` in the repo's root directory. Then you can call `cargo build` to generate the server executable.

``` bash
$ npm run build # Build front-end assets
$ cargo build
```
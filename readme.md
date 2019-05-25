# 冴えない漫画の育て方

My manga website finally comes with a rust implementation. This is not in production yet but will be soon integrated. So, I'm happy to announce that this is the 4.0 version of saemanga.com!

## Development

You can run

``` bash
$ npm run dev
```

To start the dev server. This will listen to the changes of both front-end and back-end files and recompile accordingly. So as soon as you start the dev server you don't need to touch the command line and you can simply make changes to the files as you wish. The server will be refreshing accordingly.

## Build

You can run

``` bash
$ npm run build
```

to build both the front-end and back-end. After this, you can use

``` bash
$ cargo run
```

To start the server. In this case there will be no listener for changed files and you need to start by hand if needed.

## Build for Release and Deploy

Coming soon...
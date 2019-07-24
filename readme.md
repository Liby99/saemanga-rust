# 冴えない漫画の育て方

This manga website finally comes with a rust implementation. This is not in production yet but will be soon integrated. So, I'm happy to announce that this is the 4.0 version of [saemanga.com](https://saemanga.com)!

## Development

You can run

``` bash
$ npm run dev
```

To start the dev server. This command will first try to compile the webpack and rust code. Then it will listen to the changes of both front-end and back-end files and recompile accordingly. So as soon as you start the dev server you don't need to touch the command line and you can simply make changes to the files as you wish. The server will be refreshing accordingly.

## Build for Release and Deploy

You can run

``` bash
$ npm run build-prod
```

To build the production codes. This will compile all the front-end assets to `/public`, all the template to `/templates`, and the rust executable to `/target/release`.

After this you should be thinking of uploading the code to the server. You can run

``` bash
$ ./scripts/upload.sh
```

To do the upload. Note that you still need to do a `cargo build --release` on the server so that the executable will be adapted to the server system. Finally, you can type

``` bash
$ npm run prod
```

To start the production run. Note that in this case both the server as well as the scheduler will be ran by [pm2](http://pm2.keymetrics.io) so make sure you have pm2 installed globally.

## Features

This is a manga site for Chinese manga readers since the translations are in Chinese. This manga site features scraper to [Cartoonmad](cartoonmad.com) and all the images are from that manga site. This manga site also features a minimalist design so that there's minimal overhead influencing your experience reading the manga. It will be a great experience so please try it out!

Technology wise, this manga site has

- Rust back-end powered by [Rocket](rocket.rs)
- TypeScript front-end with a self-written front-end component based library
- SCSS based stylesheets
- Handlebars templates that could be used by both front-end and back-end
- Python powered scheduler which runs simultaneously as a separate process
- Great infrastructure empowering Rust/TypeScript development
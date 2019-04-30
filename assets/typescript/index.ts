'use strict';

import "../stylesheets/index.scss"

import $ = require('jquery');

$(() => {
  $("main").prepend("Hello World");
});
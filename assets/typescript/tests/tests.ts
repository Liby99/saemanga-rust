'use strict';

// Jquery
import $ from "jquery";
import Registry from "../library/registry";

// Components
import RemoveObj from './rm_obj';

$(document).ready(() => {
  Registry.register("rm-obj", RemoveObj);
  Registry.build();
});
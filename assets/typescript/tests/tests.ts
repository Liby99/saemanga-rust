'use strict';

// Jquery
import $ from "jquery";
import Registry from "../library/registry";

// Components
import Emitter from '../controllers/helpers/emitter';
import RemoveObj from './rm_obj';
import AddCounter from './add_counter';
import Counter from './counter';

$(document).ready(() => {
  Registry.register("emitter", Emitter);
  Registry.register("rm-obj", RemoveObj);
  Registry.register("add-counter", AddCounter);
  Registry.register("counter", Counter);
  Registry.build();
});
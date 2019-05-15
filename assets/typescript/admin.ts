import '../stylesheets/admin.scss';

import $ from "jquery";
import Registry from "./library/registry";

import { Disabler } from './controllers/helpers/mod';

$(document).ready(() => {
  Registry.register("disabler", Disabler);
  Registry.build();
});
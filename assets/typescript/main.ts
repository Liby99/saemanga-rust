'use strict';

// Jquery
import $ from "jquery";

// Images and Scss
import "../images/loading.svg";
import "../stylesheets/main.scss";

// Component Library
import Registry from "./lib/registry";

// Components
import Body from "./components/body";
import Sidebar from "./components/sidebar";
import LightModeSelect from "./components/light_mode_select";
import HandModeSelect from "./components/hand_mode_select";

// Initialize all the components
$(document).ready(() => {
  Registry.register("body", Body);
  Registry.register("sidebar", Sidebar);
  Registry.register("light-mode-select", LightModeSelect);
  Registry.register("hand-mode-select", HandModeSelect);
  Registry.build();
});
'use strict';

// Jquery
import $ from "jquery";

// Images and Scss
import "../images/loading.svg";
import "../stylesheets/main.scss";

// Component Library
import Controller from "./lib/controller";

// Components
import Sidebar from "./components/sidebar";
import LightModeSelect from "./components/light_mode_select";
import HandModeSelect from "./components/hand_mode_select";

// Initialize all the components
$(document).ready(() => {
  const ctrl = new Controller();
  ctrl.register("sidebar", Sidebar);
  ctrl.register("light-mode-select", LightModeSelect);
  ctrl.register("hand-mode-select", HandModeSelect);
  ctrl.build();
});
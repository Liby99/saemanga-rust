'use strict';

// Images and Scss
import "../images/loading.svg";
import "../stylesheets/main.scss";

// Component Library
import Controller from "./lib/controller";

// Components
import Sidebar from "./components/sidebar";

// Initialize all the components
$(document).ready(() => {
  const ctrl = new Controller();
  ctrl.register("sidebar", Sidebar);
  ctrl.build();
});
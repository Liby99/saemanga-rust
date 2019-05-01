'use strict';

// Jquery
import $ from "jquery";

// Images and Scss
import "../images/loading.svg";
import "../stylesheets/main.scss";

// Component Library
import Registry from "./library/registry";

// Components
import Body from "./controllers/common/body";
import Sidebar from "./controllers/common/sidebar";
import LightModeSelect from "./controllers/common/light_mode_select";
import HandModeSelect from "./controllers/common/hand_mode_select";
import Panel from "./controllers/common/panel";
import Emitter from "./controllers/helpers/emitter";
import Discover from "./controllers/index/discover";
import Genres from "./controllers/index/genres";

// Initialize all the components
$(document).ready(() => {
  Registry.register("body", Body);
  Registry.register("sidebar", Sidebar);
  Registry.register("light-mode-select", LightModeSelect);
  Registry.register("hand-mode-select", HandModeSelect);
  Registry.register("panel", Panel);
  Registry.register("emitter", Emitter);
  Registry.register("discover", Discover);
  Registry.register("genres", Genres);
  Registry.build();
});
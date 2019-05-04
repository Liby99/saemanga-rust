'use strict';

// Jquery
import $ from "jquery";

// Import assets
import "./assets";

// Component Library
import Registry from "./library/registry";

// Components
import { Body, Sidebar, LightModeSelect, HandModeSelect, Panel } from './controllers/common/mod';
import { Emitter } from './controllers/helpers/mod';
import { Discover, Genres } from './controllers/index/mod';

// Initialize all the components
$(document).ready(() => {

  // First register all the components
  Registry.register("body", Body);
  Registry.register("sidebar", Sidebar);
  Registry.register("light-mode-select", LightModeSelect);
  Registry.register("hand-mode-select", HandModeSelect);
  Registry.register("panel", Panel);
  Registry.register("emitter", Emitter);
  Registry.register("discover", Discover);
  Registry.register("genres", Genres);

  // Then build the registry
  Registry.build();
});
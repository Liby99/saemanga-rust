'use strict';

// Jquery
import $ from "jquery";

// Import assets
import "./assets";

// Component Library
import Registry from "./library/registry";

// Components
import {
  Body,
  Sidebar,
  Login,
  LightModeSelect,
  HandModeSelect,
  Panel
} from './controllers/common/mod';
import { Emitter } from './controllers/helpers/mod';
import {
  SearchBar,
  SearchResult,
  Discover,
  Genres
} from './controllers/index/mod';

// Initialize all the components
$(document).ready(() => {

  // First register all the common components
  Registry.register("body", Body);
  Registry.register("sidebar", Sidebar);
  Registry.register("login", Login);
  Registry.register("light-mode-select", LightModeSelect);
  Registry.register("hand-mode-select", HandModeSelect);
  Registry.register("panel", Panel);
  Registry.register("emitter", Emitter);

  // Then register all the index related components
  Registry.register("search-bar", SearchBar);
  Registry.register("search-result", SearchResult);
  Registry.register("discover", Discover);
  Registry.register("genres", Genres);

  // Then build the registry
  Registry.build();
});
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
  FixBody,
  Main,
  Sidebar,
  Login,
  LightModeSelect,
  HandModeSelect,
  Zoom,
  Panel,
} from './controllers/common/mod';
import {
  Emitter,
} from './controllers/helpers/mod';
import {
  SearchBar,
  SearchResult,
  Discover,
  Genres,
} from './controllers/index/mod';
import {
  Anchorer,
  Like,
} from './controllers/manga/mod';

// Initialize all the components
$(document).ready(() => {

  // First register all the common components
  Registry.register("body", Body);
  Registry.register("fix-body", FixBody);
  Registry.register("main", Main);
  Registry.register("sidebar", Sidebar);
  Registry.register("login", Login);
  Registry.register("light-mode-select", LightModeSelect);
  Registry.register("hand-mode-select", HandModeSelect);
  Registry.register("zoom", Zoom);
  Registry.register("panel", Panel);
  Registry.register("emitter", Emitter);

  // Then register all the index related components
  Registry.register("search-bar", SearchBar);
  Registry.register("search-result", SearchResult);
  Registry.register("discover", Discover);
  Registry.register("genres", Genres);

  // Then register all the manga related components
  Registry.register("anchorer", Anchorer);
  Registry.register("like", Like);

  // Then build the registry
  Registry.build();
});
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
  SidebarToggle,
  Login,
  LightModeSelect,
  HandModeSelect,
  Zoom,
  Panel,
} from './controllers/common/mod';
import {
  Emitter,
  StickTop,
} from './controllers/helpers/mod';
import {
  SearchBar,
  SearchResult,
  Discover,
  Genres,
  LikedOnly,
  FollowingHeader,
  FollowingManageToggle,
  FollowingManga,
  FollowingList,
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
  Registry.register("sidebar-toggle", SidebarToggle);
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
  Registry.register("liked-only", LikedOnly);
  Registry.register("stick-top", StickTop);
  Registry.register("following-header", FollowingHeader);
  Registry.register("following-manage-toggle", FollowingManageToggle);
  Registry.register("following-manga", FollowingManga);
  Registry.register("following-list", FollowingList);

  // Then register all the manga related components
  Registry.register("anchorer", Anchorer);
  Registry.register("like", Like);

  // Then build the registry
  Registry.build();
});
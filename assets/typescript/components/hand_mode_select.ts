import Select from "./select";
import EventPool from "../lib/event_pool";

export default class HandModeSelect extends Select {
  selectLeft() {
    super.selectLeft();
    EventPool.dispatch("settings.hand_mode.change", "left");
  }

  selectRight() {
    super.selectRight();
    EventPool.dispatch("settings.hand_mode.change", "right");
  }
}
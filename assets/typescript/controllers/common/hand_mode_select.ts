import Select from "./select";
import EventPool from "../../library/event_pool";

export default class HandModeSelect extends Select {
  selectLeft() {
    super.selectLeft();
    EventPool.emit("settings.hand_mode.change", "left");
  }

  selectRight() {
    super.selectRight();
    EventPool.emit("settings.hand_mode.change", "right");
  }
}
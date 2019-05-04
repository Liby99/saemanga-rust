import Select from "./select";
import EventPool from "../../library/event_pool";

export default class LightModeSelect extends Select {
  selectLeft() {
    super.selectLeft();
    EventPool.emit("settings.light_mode.change", "day");
  }

  selectRight() {
    super.selectRight();
    EventPool.emit("settings.light_mode.change", "night");
  }
}
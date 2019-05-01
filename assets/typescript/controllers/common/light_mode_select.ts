import Select from "./select";
import EventPool from "../../library/event_pool";

export default class LightModeSelect extends Select {
  selectLeft() {
    super.selectLeft();
    EventPool.dispatch("settings.light_mode.change", "day");
  }

  selectRight() {
    super.selectRight();
    EventPool.dispatch("settings.light_mode.change", "night");
  }
}
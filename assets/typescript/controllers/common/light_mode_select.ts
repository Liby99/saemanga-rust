import Axios from "axios";

import Select from "./select";
import EventPool from "../../library/event_pool";

export default class LightModeSelect extends Select {
  selectLeft() {
    this.setLightMode("day", () => {
      super.selectLeft();
    });
  }

  selectRight() {
    this.setLightMode("night", () => {
      super.selectRight();
    });
  }

  setLightMode(mode: "day" | "night", callback: () => void) {
    Axios.post("/ajax/set_light_mode", { mode }).then((response) => {
      EventPool.emit("settings.light_mode.change", mode);
      callback();
    });
  }
}
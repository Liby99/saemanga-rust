import Axios from "axios";

import Select from "./select";
import EventPool from "../../library/event_pool";

export default class HandModeSelect extends Select {
  selectLeft() {
    this.setHandMode("left", () => {
      super.selectLeft();
    });
  }

  selectRight() {
    this.setHandMode("right", () => {
      super.selectRight();
    });
  }

  setHandMode(mode: "left" | "right", callback: () => void) {
    Axios.post("/ajax/set_hand_mode", { mode }).then((response) => {
      EventPool.emit("settings.hand_mode.change", mode);
      callback();
    });
  }
}
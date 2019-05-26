import Axios from 'axios';

import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

type State = {
  isMin: boolean,
  isMax: boolean,
  scale: number,
}

export default class Zoom extends Controller<State> {

  static STEP : number = 0.15;
  static UPDATE_COOKIE_DELAY : number = 3000;

  updateTimeout : number | undefined;

  $minus: JQuery<HTMLElement>;
  $data: JQuery<HTMLElement>;
  $plus: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.$minus = root.children(".minus");
    this.$data = root.children(".data");
    this.$plus = root.children(".plus");

    this.$minus.click(() => {
      const { scale } = this.state;
      EventPool.emit("setting.scale.set", scale - Zoom.STEP);
    });

    this.$data.click(() => {
      EventPool.emit("setting.scale.set", 1);
    });

    this.$plus.click(() => {
      const { scale } = this.state;
      EventPool.emit("setting.scale.set", scale + Zoom.STEP);
    });

    this.listen("setting.scale.update", (data: State) => {
      this.setState(data);
      this.updateCookies();
    });
  }

  initialState() : State {
    return {
      isMin: false,
      isMax: false,
      scale: parseInt(this.root.children(".data").text()) / 100.0,
    };
  }

  updateCookies() {

    // To prevent calling backend too many times, we use timeout and delay here
    // It will wait for `UPDATE_COOKIE_DELAY` milliseconds, and if within this
    // time frame there's still update, we will cancel the previous schedule and
    // create a new one also waiting for `UPDATE_COOKIE_DELAY` milliseconds.
    // After all this will be run after `UPDATE_COOKIE_DELAY` milliseconds of
    // unaction.
    if (this.updateTimeout !== undefined) {
      clearTimeout(this.updateTimeout)
    }

    // Really try to update
    this.updateTimeout = setTimeout(() => {

      // Use Axios to post a request
      const { scale } = this.state;
      Axios.post("/ajax/set_scale", { scale }).then((_) => {});

      // Change it back to undefined
      this.updateTimeout = undefined;
    }, Zoom.UPDATE_COOKIE_DELAY);
  }

  update() {
    const { isMin, isMax, scale } = this.state;

    if (isMax) this.$plus.addClass("disabled");
    else this.$plus.removeClass("disabled");

    if (isMin) this.$minus.addClass("disabled");
    else this.$minus.removeClass("disabled");

    this.$data.text(`${Math.round(scale * 100)}`);
  }
}
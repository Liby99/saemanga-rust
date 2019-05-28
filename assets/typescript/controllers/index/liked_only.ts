import Axios, { AxiosResponse } from 'axios';

import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

type State = {
  isLikedOnly: boolean,
};

type Response = {
  success: boolean,
  message: string,
};

export default class LikedOnly extends Controller<State> {

  $toggle: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.$toggle = this.root.children("i");

    this.root.click(() => {
      const { isLikedOnly } = this.state;
      const newState = !isLikedOnly;
      Axios.post("/ajax/set_liked_only", { mode: this.toString(newState) }).then((response: AxiosResponse<Response>) => {
        const { success, message } = response.data;
        if (success) {
          EventPool.emit("settings.liked_only.change", newState);
          this.setState({ isLikedOnly: newState });
        } else {
          alert(message);
        }
      });
    });
  }

  initialState() : State {
    return { isLikedOnly: this.root.hasClass("active") };
  }

  toString(isLikedOnly: boolean) : "on" | "off" {
    return isLikedOnly ? "on" : "off";
  }

  update() {
    const { isLikedOnly } = this.state;
    if (isLikedOnly) {
      this.root.addClass("active");
      this.$toggle.addClass("active");
    } else {
      this.root.removeClass("active");
      this.$toggle.removeClass("active");
    }
  }
}
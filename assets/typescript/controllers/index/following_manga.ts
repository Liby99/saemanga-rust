import Axios, { AxiosResponse } from 'axios';

import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

type State = {
  showing: boolean,
};

type UnfollowResponse = {
  success: boolean,
  message: string,
};

export default class FollowingManga extends Controller<State> {

  title: string;
  dmkId: string;
  isLiked: boolean;

  $cover: JQuery<HTMLElement>;
  $remove: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Parameters
    this.isLiked = this.root.hasClass("liked");
    this.title = this.root.find(".title").text();
    this.dmkId = this.root.attr("data-dmk-id") || "";

    // Remove Element
    this.$cover = this.root.children(".cover");
    this.$remove = this.$cover.children(".remove");

    // Add animation delay
    this.$cover.css({ "animation-delay": `${-Math.random()}s` });

    // Remove handler
    this.$remove.click((event) => {
      if (confirm(`您确定要取消关注 ${this.title} 吗？`)) {
        Axios.get(`/ajax/manga/unfollow?dmk_id=${this.dmkId}`).then((response: AxiosResponse<UnfollowResponse>) => {
          const { success, message } = response.data;
          if (success) {
            EventPool.emit("following.unfollow", this.root[0]);
          } else {
            alert(message);
          }
        });
      }
      event.preventDefault();
    });

    this.listen("following.manage", (managing: boolean) => {
      if (managing) {
        this.root.addClass("managing");
      } else {
        this.root.removeClass("managing");
      }
    });

    this.listen("settings.liked_only.change", (isLikedOnly: boolean) => {
      this.setState({ showing: this.isLiked || !isLikedOnly });
    });
  }

  initialState() : State {
    return { showing: !this.root.attr("hidden") };
  }

  update() {
    const { showing } = this.state;
    if (showing) {
      this.root.removeAttr("hidden");
    } else {
      this.root.attr("hidden", "hidden");
    }
  }
}
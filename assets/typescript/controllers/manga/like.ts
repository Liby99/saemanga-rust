import Axios, { AxiosResponse } from 'axios';

import Controller from '../../library/controller';

type State = {
  liked: boolean,
};

type Response = {
  success: boolean,
  message: string,
};

export default class Like extends Controller<State> {

  dmkId: string;

  $like: JQuery<HTMLElement>;
  $unlike: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.dmkId = this.root.attr("data-dmk-id") || "";

    this.$like = root.children("#like");
    this.$unlike = root.children("#unlike");

    this.root.click(() => {
      const { liked } = this.state;
      const command = liked ? "unlike" : "like";
      Axios.get(`/ajax/manga/${command}?dmk_id=${this.dmkId}`).then((response: AxiosResponse<Response>) => {
        const { success, message } = response.data;
        if (success) {
          this.setState({ liked: !liked });
        } else {
          alert(message);
        }
      });
    });
  }

  initialState() : State {
    const hiddenId = this.root.children("[hidden]").attr("id");
    return { liked: hiddenId === "like" };
  }

  update() {
    const { liked } = this.state;
    if (liked) {
      this.$like.attr("hidden", "hidden");
      this.$unlike.removeAttr("hidden");
    } else {
      this.$like.removeAttr("hidden");
      this.$unlike.attr("hidden", "hidden");
    }
  }
}
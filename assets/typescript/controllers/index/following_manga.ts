import Controller from '../../library/controller';

type State = {
  showing: boolean,
};

export default class FollowingManga extends Controller<State> {

  isLiked: boolean;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.isLiked = this.root.hasClass("liked");

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
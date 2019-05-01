import Component from '../lib/component';
import EventPool from '../lib/event_pool';

type BodyState = {
  isLeftHandMode: boolean,
  isNightMode: boolean,
};

export default class Body extends Component<BodyState> {
  constructor(root: JQuery<HTMLElement>) {
    super(root);

    EventPool.listen("settings.hand_mode.change", (mode: string) => {
      switch (mode) {
        case "left": this.setLeftHandMode(); break;
        case "right": this.setRightHandMode(); break;
      }
    });
  }

  initialState(root: JQuery<HTMLElement>) : BodyState {
    return {
      isLeftHandMode: root.hasClass("left"),
      isNightMode: root.hasClass("night"),
    };
  }

  setLeftHandMode() {
    this.setState({ isLeftHandMode: true });
  }

  setRightHandMode() {
    this.setState({ isLeftHandMode: false });
  }

  update() {
    const { isLeftHandMode } = this.state;
    if (isLeftHandMode) {
      this.root.addClass("left");
    } else {
      this.root.removeClass("left");
    }
  }
}
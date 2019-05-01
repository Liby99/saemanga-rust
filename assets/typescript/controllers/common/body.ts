import Controller from '../../library/controller';

type BodyState = {
  isLeftHandMode: boolean,
  isNightMode: boolean,
};

export default class Body extends Controller<BodyState> {
  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.listen("settings.hand_mode.change", (mode: string) => {
      switch (mode) {
        case "left": this.setLeftHandMode(); break;
        case "right": this.setRightHandMode(); break;
      }
    });

    this.listen("settings.light_mode.change", (mode: string) => {
      switch (mode) {
        case "day": this.setDayLightMode(); break;
        case "night": this.setNightLightMode(); break;
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

  setDayLightMode() {
    this.setState({ isNightMode: false });
  }

  setNightLightMode() {
    this.setState({ isNightMode: true });
  }

  update() {
    const { isLeftHandMode, isNightMode } = this.state;

    // Deal with hand mode
    if (isLeftHandMode) {
      this.root.addClass("left");
    } else {
      this.root.removeClass("left");
    }

    // Deal with light mode
    if (isNightMode) {
      this.root.addClass("night");
    } else {
      this.root.removeClass("night");
    }
  }
}
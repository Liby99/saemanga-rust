import Controller from '../../library/controller';

type BodyState = {
  isLeftHandMode: boolean,
  isNightMode: boolean,
  isFullWidth: boolean,
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

    this.listen("setting.scale.update", (result: { isMax: boolean }) => {
      const { isMax } = result;
      this.setState({ isFullWidth: isMax });
    });
  }

  initialState() : BodyState {
    return {
      isLeftHandMode: this.root.hasClass("left"),
      isNightMode: this.root.hasClass("night"),
      isFullWidth: false,
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
    const { isLeftHandMode, isNightMode, isFullWidth } = this.state;

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

    if (isFullWidth) {
      this.root.addClass("full-width");
    } else {
      this.root.removeClass("full-width");
    }
  }
}
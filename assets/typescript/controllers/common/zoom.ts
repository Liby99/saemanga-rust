import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

type State = {
  isMin: boolean,
  isMax: boolean,
  scale: number,
}

export default class Zoom extends Controller<State> {

  STEP : number = 0.15;

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
      EventPool.emit("setting.scale.set", scale - this.STEP);
    });

    this.$data.click(() => {
      EventPool.emit("setting.scale.set", 1);
    });

    this.$plus.click(() => {
      const { scale } = this.state;
      EventPool.emit("setting.scale.set", scale + this.STEP);
    });

    this.listen("setting.scale.update", (data: State) => {
      this.setState(data);
    });
  }

  initialState() : State {
    return {
      isMin: false,
      isMax: false,
      scale: parseInt(this.root.children(".data").text()) / 100.0,
    };
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
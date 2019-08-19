import $ from 'jquery';

import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

const THRESHOLD = 5;

type State = {
  width: number,
  actualWidth: number,
};

function getWindowWidth() : number {
  return $(window).width() || 0;
}

export default class Main extends Controller<State> {

  static ABSOLUTE_MIN_WIDTH : number = 320;
  static ABSOLUTE_DEFAULT_WIDTH : number = 768;

  baseWidth: number = 0;
  maxWidth: number = 0;
  minWidth: number = 0;

  windowWidth: number = 0;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // First update the frame
    this.updateFrame();

    // Listen to resize event to refresh the frame
    $(window).resize(() => {

      // Filter out all the events that width does not change
      if (getWindowWidth() !== this.windowWidth && getWindowWidth() > THRESHOLD) {
        this.updateFrame();
      }
    });

    // Also listen to set scale event
    EventPool.listen("setting.scale.set", (scale: number) => {
      const newWidth = this.baseWidth * scale;
      this.setState({
        width: newWidth,
        actualWidth: this.clamp(newWidth),
      });
      this.emitWidthSetEvent();
    });
  }

  initialState() : State {
    const w = this.root.width() || 0;
    return { width: w, actualWidth: w };
  }

  updateFrame() {

    // First update the frame
    this.windowWidth = getWindowWidth();
    this.baseWidth = Math.min(Main.ABSOLUTE_DEFAULT_WIDTH, this.windowWidth);

    // Max width is the window width
    this.maxWidth = this.windowWidth;

    // Min width need to be absolutely > 320
    // It has to be < 768
    // And we pick windowWidth / 2 for the middle
    this.minWidth = Math.max(Main.ABSOLUTE_MIN_WIDTH, Math.min(this.windowWidth / 2.0, Main.ABSOLUTE_DEFAULT_WIDTH));

    // Clamp actual width
    const { width } = this.state;
    if (width > this.maxWidth) this.setState({ width: this.maxWidth, actualWidth: this.maxWidth });
    if (width < this.minWidth) this.setState({ width: this.minWidth, actualWidth: this.minWidth });

    // Then send out update message
    this.emitWidthSetEvent();
  }

  emitWidthSetEvent() {
    const { width, actualWidth } = this.state;
    EventPool.emit("setting.scale.update", {
      isMin: width <= this.minWidth,
      isMax: width >= this.maxWidth,
      scale: actualWidth / this.baseWidth,
    });
  }

  clamp(width: number) {
    return Math.min(Math.max(width, this.minWidth), this.maxWidth);
  }

  update() {
    const { actualWidth } = this.state;
    this.root.css("max-width", `${actualWidth}px`);
  }
}
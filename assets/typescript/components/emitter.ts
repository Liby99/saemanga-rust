import Component from '../lib/component';
import EventPool from '../lib/event_pool';

export default class Emitter extends Component<{}> {

  events: string[];

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Get events
    const attr = root.attr("events") || "";
    this.events = attr.split(",").map(s => s.trim()).filter(s => s !== "");

    // Events
    this.root.click(() => {
      this.events.forEach((e) => EventPool.dispatch(e));
    });
  }

  initialState() : {} {
    return {};
  }
}
import Controller from '../library/controller';
// import Registry from '../library/registry';
import EventPool from '../library/event_pool';

let count = 0;

export default class RemoveObj extends Controller<{}> {
  id: number;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.id = count;
    root.text(`Rm Obj #${count++}`);

    root.click(() => { root.remove(); });

    this.listen("dummy-event", () => {});
  }

  initialState() : {} { return {} }

  onDestroy() {
    alert(`Rm Obj #${this.id} being removed`);

    alert(`event listener remaining: ${EventPool.events['dummy-event'].length}`);
  }
}
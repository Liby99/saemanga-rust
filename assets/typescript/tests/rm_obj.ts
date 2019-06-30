import Controller from '../library/controller';

let count = 0;

export default class RemoveObj extends Controller<{}> {
  id: number;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.id = count;
    root.text(`Rm Obj #${count++}`);

    root.click(() => { root.remove(); });
  }

  initialState() : {} { return {} }

  onDestroy() {
    alert(`Rm Obj #${this.id} being removed`);
  }
}
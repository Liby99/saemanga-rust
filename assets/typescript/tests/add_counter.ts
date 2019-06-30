import Controller from '../library/controller';
import CounterTemplate from '../templates/counter';

export default class AddCounter extends Controller<{}> {
  constructor(root: JQuery<HTMLElement>) {
    super(root);
    this.listen("counter.add", () => {
      CounterTemplate.appendTo({}, root);
    });
  }

  initialState() : {} { return {} }
}
import Controller from '../../library/controller';

export default class Disabler extends Controller<{}> {

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.root.submit(() => {
      this.root.find("button").attr("disabled", "true");
    });
  }

  initialState() : {} {
    return {};
  }
}
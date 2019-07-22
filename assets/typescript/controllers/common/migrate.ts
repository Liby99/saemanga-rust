import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

export default class Migrate extends Controller<{}> {
  constructor(root: JQuery<HTMLElement>) {
    super(root);

    setTimeout(() => {
      EventPool.emit("panel.migrate.open");
    }, 3000);
  }

  initialState() : {} {
    return {};
  }
}
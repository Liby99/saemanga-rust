import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

export default class Login extends Controller<{}> {

  $username: JQuery<HTMLInputElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.$username = root.find("#login-username") as JQuery<HTMLInputElement>;

    this.listen("login.focus", () => {
      this.$username.focus();
      EventPool.emit("sidebar.open");
    });
  }

  initialState() : {} {
    return {};
  }
}
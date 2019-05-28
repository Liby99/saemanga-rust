import EventPool from '../../library/event_pool';
import StickTop from '../helpers/stick_top';

export default class FollowingHeader extends StickTop {

  $inner: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);
    this.$inner = this.$content.children();
    this.listen("setting.scale.update", () => this.resize());
  }

  onRelease() {
    this.$inner.removeClass("stick");
    EventPool.emit("following.header.release");
  }

  onStick() {
    this.$inner.addClass("stick");
    EventPool.emit("following.header.stick");
  }
}
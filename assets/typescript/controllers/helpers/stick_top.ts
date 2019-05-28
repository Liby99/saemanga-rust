import $ from 'jquery';

import Controller from '../../library/controller';

type State = {};

export default class StickTop extends Controller<State> {

  $placeholder: JQuery<HTMLElement>;
  $content: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Initialize
    this.$placeholder = root.children(".placeholder");
    this.$content = root.children(".content");
    this.$placeholder.height(this.$content.outerHeight() || 0);

    // Listeners
    $(window).scroll(() => this.refresh());
    $(window).resize(() => this.resize());
  }

  initialState() : State {
    return {};
  }

  refresh() {
    const off = this.root.offset();
    if ((off ? off.top : 0) < ($(window).scrollTop() || 0)) {
      this.stick();
    } else {
      this.release();
    }
  }

  stick() {
    this.$placeholder.css({ display: "block" });
    this.$content.css({ position: "fixed", top: 0, width: this.root.width() || 0 });
    if (!this.root.hasClass("stick")) {
      this.root.addClass("stick");
      this.onStick();
    }
  }

  release() {
    this.$placeholder.css({ display: "none" });
    this.$content.css({ position: "", top: "", width: "" });
    if (this.root.hasClass("stick")) {
      this.root.removeClass("stick");
      this.onRelease();
    }
  }

  resize() {
    if (this.root.hasClass("stick")) {
      this.$content.css({ width: this.root.width() || 0 });
    }
  }

  // Virtual function for override
  onRelease() {}

  // Virtual function for override
  onStick() {}
}
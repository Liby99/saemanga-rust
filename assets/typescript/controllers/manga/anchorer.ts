import $ from 'jquery';

import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

type State = {
  page: number,
  percInPage: number,
};

export default class Anchorer extends Controller<State> {

  static MARGIN : number = 5;

  sectionToTop : number = 0;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    $(window).scroll(() => {
      if (!$("body").hasClass("fixed")) {
        const scrollTop = $(window).scrollTop() || 0;
        this.setState($.makeArray(this.root.children()).reduce((state, elem, index) => {
          const off = $(elem).position(), top = off ? off.top : 0;
          const height = $(elem).height() || 0;
          if (scrollTop >= top && scrollTop <= top + height + Anchorer.MARGIN) {
            return { page: index, percInPage: (scrollTop - top) / height };
          } else {
            return state;
          }
        }, this.initialState()));
      }
    });

    this.listen("setting.scale.update", () => {
      const { page, percInPage } = this.state;
      if (page !== -1) {
        const $elem = this.root.children().eq(page);
        const off = $elem.position(), top = off ? off.top : 0;
        const height = $elem.height() || 0;
        const scrollTop = top + height * percInPage;
        EventPool.emit("body.fix_at", scrollTop);
      }
    });
  }

  initialState() : State {
    return {
      page: -1,
      percInPage: 0,
    };
  }
}
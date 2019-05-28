import MixItUp from 'mixitup';

import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

import NotFollowingMangaTemplate from '../../templates/not_following_manga';

export default class FollowingList extends Controller<{}> {
  mixer: any;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.mixer = MixItUp(`#${this.root.attr("id")}`);

    this.listen("following.unfollow", (manga: HTMLElement) => {
      this.mixer.remove(manga).then(() => {
        this.checkNoFollow();
      });
    });
  }

  initialState() : {} {
    return {};
  }

  checkNoFollow() {
    if (this.root.children().length === 0) {
      EventPool.emit("following.manage.finish");
      this.root.hide(0);
      this.root.html(NotFollowingMangaTemplate.render({}));
      this.root.slideDown(300);
    }
  }
}
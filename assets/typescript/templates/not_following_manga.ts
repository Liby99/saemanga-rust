import hbs from '../../templates/partials/index/not_following_manga.html.hbs';
import Template from '../library/template';

export class NotFollowingMangaTemplate extends Template<{}> {
  constructor() {
    super(hbs);
  }
}

export default new NotFollowingMangaTemplate();
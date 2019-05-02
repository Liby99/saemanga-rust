import hbs from '../../templates/partials/index/discover_manga.html.hbs';
import Template from '../library/template';

export type DiscoverMangaData = {
  title: string,
  dmk_id: string,
  cover_url: string,
  saemanga_url: string,
};

export class DiscoverMangaTemplate extends Template<DiscoverMangaData> {
  constructor() {
    super(hbs);
  }
}

export default new DiscoverMangaTemplate();
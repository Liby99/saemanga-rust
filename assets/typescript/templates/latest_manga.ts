import hbs from '../../templates/partials/latest_manga.html.hbs';
import Template from '../library/template';

export type LatestMangaData = {
  title: string,
  dmk_id: string,
  cover_url: string,
  saemanga_url: string,
};

export class LatestMangaTemplate extends Template<LatestMangaData> {
  constructor() {
    super(hbs);
  }
}

export default new LatestMangaTemplate();
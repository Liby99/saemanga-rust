import Axios, { AxiosResponse } from 'axios';

import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';
import Chinese from '../../library/chinese';
import DiscoverManga, { DiscoverMangaData } from '../../templates/discover_manga';
import { Discover } from './mod';

type State = {
  result: DiscoverMangaData[],
};

type SearchResponse = {
  success: boolean,
  message: string,
  data: DiscoverMangaData[],
};

type FetchCallback = (err: Error | undefined, result: DiscoverMangaData[]) => void;

export default class SearchResult extends Controller<State> {
  $outer: JQuery<HTMLElement>;
  $result: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.$outer = root;
    this.$result = this.$outer.children();

    this.listen("search.submit", (searchText: string) => {
      this.fetch(searchText, (err, mangas) => {
        if (err) {
          this.setState({ result: [] }, () => {
            EventPool.emit("search.result.failed", new Error("抱歉，服务器出错了。"));
          });
        } else {
          if (mangas.length) {
            EventPool.emit("search.result.fetched", mangas);
            this.setState({ result: mangas }, () => {
              this.scrollToLeft();
              EventPool.emit("search.result.opened");
            });
          } else {
            this.setState({ result: [] }, () => {
              EventPool.emit("search.result.failed", new Error("抱歉，您搜索的漫画未能找到。"));
            });
          }
        }
      });
    });

    this.listen("search.result.close", () => {
      this.setState({ result: [] }, () => {
        EventPool.emit("search.result.closed");
      });
    });
  }

  initialState() : State {
    return { result: [] };
  }

  fetch(searchText: string, callback: FetchCallback) {
    const traditionalized = Chinese.traditionalize(searchText);
    Axios.get(`/ajax/search?text=${traditionalized}`).then((response: AxiosResponse<SearchResponse>) => {
      const data = response.data;
      if (data.success) {
        callback(undefined, data.data);
      } else {
        callback(new Error(data.message), []);
      }
    }).catch((reason) => {
      callback(reason, []);
    });
  }

  scrollToLeft() {
    this.$outer.animate({ scrollLeft: 0 }, 500);
  }

  updateResult(callback: () => void) {
    const { result } = this.state;
    DiscoverManga.mountTo(result, this.$result);
    callback();
  }

  update(callback: () => void) {
    const { result } = this.state;
    if (result.length) {
      DiscoverManga.mountTo(result, this.$result);
      this.$outer.slideDown(200, callback);
    } else {
      this.$outer.slideUp(200, () => {
        this.$result.html("");
        callback();
      });
    }
  }
}
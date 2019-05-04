import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';
import DiscoverManga, { DiscoverMangaData } from '../../templates/discover_manga';

type SearchResultData = {
  result: DiscoverMangaData[],
};

export default class SearchResult extends Controller<SearchResultData> {
  $outer: JQuery<HTMLElement>;
  $result: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.$outer = root;
    this.$result = this.$outer.children();

    this.listen("search.submit", (searchText: string) => {
      this.fetch(searchText, (err, mangas) => {
        if (err) {
          EventPool.emit("search.result.failed", err);
        } else {
          if (mangas.length) {
            EventPool.emit("search.result.fetched", mangas);
            this.setState({ result: mangas }, () => {
              EventPool.emit("search.result.opened");
            });
          } else {
            EventPool.emit("search.result.failed", new Error("抱歉，您搜索的漫画未能找到。"));
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

  initialState() : SearchResultData {
    return { result: [] };
  }

  fetch(searchText: string, callback: (err: any, result: DiscoverMangaData[]) => void) {
    setTimeout(() => {
      callback(undefined, [{
        title: "五等分的花嫁",
        dmk_id: "5893",
        cover_url: "http://cartoonmad.com/cartoonimg/coimg/5893.jpg",
        saemanga_url: "http://saemanga.com/manga/5893",
      }, {
        title: "雖然我也想脫宅",
        dmk_id: "7702",
        cover_url: "http://cartoonmad.com/cartoonimg/coimg/7702.jpg",
        saemanga_url: "http://saemanga.com/manga/7702",
      }]);
    }, 1000);
  }

  updateResult(callback: () => void) {
    const { result } = this.state;
    this.$result.html(DiscoverManga.render(result));
    callback();
  }

  update(callback: () => void) {
    const { result } = this.state;
    if (result.length) {
      this.$result.html(DiscoverManga.render(result));
      this.$outer.slideDown(200, callback);
    } else {
      this.$outer.slideUp(200, () => {
        this.$result.html("");
        callback();
      });
    }
  }
}
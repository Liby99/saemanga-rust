import $ from 'jquery';
import Controller from '../../library/controller';
import LatestManga, { LatestMangaData } from '../../templates/latest_manga';

enum ShowingState {
  Hiding,
  Rendering,
  Showing,
}

type DiscoverState = {
  show: boolean,
  mangas: LatestMangaData[],
};

export default class Discover extends Controller<DiscoverState> {

  $outer: JQuery<HTMLElement>;
  $inner: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Elements
    this.$outer = this.root;
    this.$inner = this.root.children();

    // Listener
    this.listen("discover.genre.change", (genre: string) => {
      let mangas: LatestMangaData[] | undefined = undefined;
      let hidden = false;
      const next = () => {
        if (mangas && hidden) {
          this.setState({ mangas, show: true }, () => {
            this.scrollToLeft();
          });
        }
      };
      this.setState({ show: false }, () => {
        hidden = true;
        next();
      });
      this.fetch(genre, (ms) => {
        mangas = ms;
        next();
      });
    });
  }

  initialState(root: JQuery<HTMLElement>) : DiscoverState {
    return {
      show: true,
      mangas: root.children().children().toArray().map((elem) => {
        const $elem = $(elem);
        const bgimg = $elem.find("div").css("background-image");
        return {
          title: $elem.find("h4").text(),
          dmk_id: $elem.attr("dmk_id") || "",
          saemanga_url: $elem.attr("href") || "",
          cover_url: bgimg.substr(5, bgimg.length - 2),
        };
      }),
    };
  }

  fetch(genre: string, callback: (mangas: LatestMangaData[]) => void) {
    callback([{
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
  }

  scrollToLeft() {
    this.$outer.animate({ scrollLeft: 0 }, 500);
  }

  update(callback: () => void) {
    const { mangas, show } = this.state;

    this.$inner.html(LatestManga.render(mangas));

    if (show) {
      this.$outer.animate({ "opacity": 1 }, 150, callback);
    } else {
      this.$outer.animate({ "opacity": 0 }, 150, callback);
    }
  }
}
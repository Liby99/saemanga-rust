import $ from 'jquery';
import Axios, { AxiosResponse } from 'axios';

import Controller from '../../library/controller';
import DiscoverManga, { DiscoverMangaData } from '../../templates/discover_manga';

type DiscoverState = {
  show: boolean,
  mangas: DiscoverMangaData[],
};

type GetLatestMangaResponse = {
  success: boolean,
  message: string,
  data: Array<DiscoverMangaData>,
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
      let mangas: DiscoverMangaData[] | undefined = undefined;
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

  initialState() : DiscoverState {
    return {
      show: true,
      mangas: this.root.children().children().toArray().map((elem) => {
        const $elem = $(elem);
        const bgimg = $elem.find("div").css("background-image");
        return {
          "title": $elem.find("h4").text(),
          "dmk_id": $elem.attr("dmk_id") || "",
          "saemanga_url": $elem.attr("href") || "",
          "cover_url": bgimg.substr(5, bgimg.length - 2),
        };
      }),
    };
  }

  fetch(genre: string, callback: (mangas: DiscoverMangaData[]) => void) {
    Axios.get(`/ajax/get_latest_manga${genre == "" ? "" : `?genre=${genre}`}`).then((response: AxiosResponse<GetLatestMangaResponse>) => {
      const { success, message, data } = response.data;
      if (success) {
        callback(data);
      } else {
        alert(message);
      }
    }).catch((err) => {
      alert("Cannot connect to the server");
    });
  }

  scrollToLeft() {
    this.$outer.animate({ scrollLeft: 0 }, 500);
  }

  update(callback: () => void) {
    const { mangas, show } = this.state;

    DiscoverManga.mount(mangas, this.$inner);

    if (show) {
      this.$outer.animate({ "opacity": 1 }, 150, callback);
    } else {
      this.$outer.animate({ "opacity": 0 }, 150, callback);
    }
  }
}
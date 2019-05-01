import $ from 'jquery';
import Controller from '../../library/controller';
import LatestManga, { LatestMangaData } from '../../templates/latest_manga';

type DiscoverState = {
  genreListOpened: boolean,
  genre: string,
  mangas: LatestMangaData[],
};

export default class Discover extends Controller<DiscoverState> {

  $tagsToggle: JQuery<HTMLElement>;
  $tagsHolder: JQuery<HTMLElement>;
  $tags: JQuery<HTMLElement>;
  $latestMangas: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Elements
    this.$tagsToggle = root.find("#index-discover-tags-toggle");
    this.$tagsHolder = root.find("#index-discover-tags-holder");
    this.$tags = root.find("#index-discover-tags");
    this.$latestMangas = root.find("#index-discover-content");

    // Events
    this.$tags.each((_, tag) => {
      $(tag).click(() => {
        const genre = $(this).attr("id");
        if (genre) {
          this.toggleGenre(genre);
        } else {
          console.error(`Tag ${$(this).text()} doesn't have genre id`);
        }
      });
    });
    this.$tagsToggle.click(() => this.toggleGenreList());
  }

  initialState(root: JQuery<HTMLElement>) : DiscoverState {
    return {
      genre: "",
      genreListOpened: false,
      mangas: [],
    };
  }

  openGenreList() {
    this.setState({ genreListOpened: true });
  }

  closeGenreList() {
    this.setState({ genreListOpened: false });
  }

  toggleGenreList() {
    const { genreListOpened } = this.state;
    this.setState({ genreListOpened: !genreListOpened });
  }

  toggleGenre(genre: string) {
    const { genre: currGenre } = this.state;
    if (currGenre == genre) {
      this.setState({ genre: "" });
    } else {
      this.setState({ genre });
    }
  }

  update() {
    const { genre, genreListOpened, mangas } = this.state;

    if (genre == "") {
      this.$tags.each((_, elem) => {
        $(elem).removeClass("active");
      });
    } else {
      this.$tags.each((_, elem) => {
        if ($(elem).attr("id") == genre) {
          $(elem).addClass("active");
        } else {
          $(elem).removeClass("active");
        }
      });
    }

    if (genreListOpened) {
      this.$tagsToggle.addClass("active");
      this.$tagsHolder.addClass("active");
    } else {
      this.$tagsToggle.removeClass("active");
      this.$tagsHolder.removeClass("active");
    }

    this.$latestMangas.append(LatestManga.render({
      title: "雖然我也想脫宅",
      dmk_id: "7702",
      cover_url: "http://cartoonmad.com/cartoonimg/coimg/7702.jpg",
      saemanga_url: "http://saemanga.com/manga/7702"
    }));
  }
}
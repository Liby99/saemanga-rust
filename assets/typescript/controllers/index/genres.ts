import $ from 'jquery';
import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

type GenresState = {
  opened: boolean,
  selected: string,
};

export default class Genres extends Controller<GenresState> {

  $tagsToggle: JQuery<HTMLElement>;
  $tagsHolder: JQuery<HTMLElement>;
  $tagsOuter: JQuery<HTMLElement>;
  $tags: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Elements
    this.$tagsToggle = root.find("#index-discover-tags-toggle");
    this.$tagsHolder = root.find("#index-discover-tags-holder");
    this.$tagsOuter = root.find("#index-discover-tags");
    this.$tags = this.$tagsOuter.children();

    // Events
    this.$tags.each((_, tag) => {
      const genre = $(tag).attr("id");
      if (genre) $(tag).click(() => this.toggleGenre(genre));
    });
    this.$tagsToggle.click(() => this.toggleGenreList());
  }

  initialState() : GenresState {
    return { opened: false, selected: "" };
  }

  openGenreList() {
    this.setState({ opened: true });
  }

  closeGenreList() {
    this.setState({ opened: false });
  }

  toggleGenreList() {
    const { opened } = this.state;
    this.setState({ opened: !opened });
  }

  toggleGenre(genre: string) {
    const { selected: currGenre } = this.state;
    const nextGenre = currGenre == genre ? "" : genre;
    this.setState({ selected: nextGenre });
    EventPool.dispatch("discover.genre.change", nextGenre);
  }

  tagScrollToRight() {
    this.$tagsHolder.animate({
      scrollLeft: this.$tagsOuter.width()
    });
  }

  clearTagHolderCss() {
    this.$tagsHolder.css({
      width: "",
      marginRight: "",
      overflow: ""
    });
  }

  update() {
    const { selected, opened } = this.state;

    if (selected == "") {
      this.$tags.each((_, elem) => {
        $(elem).removeClass("active");
      });
    } else {
      this.$tags.each((_, elem) => {
        if ($(elem).attr("id") == selected) {
          $(elem).addClass("active");
        } else {
          $(elem).removeClass("active");
        }
      });
    }

    if (opened) {
      this.clearTagHolderCss();
      this.$tagsToggle.addClass("active");
      this.$tagsHolder.addClass("active");
      this.tagScrollToRight();
    } else {
      this.$tagsToggle.removeClass("active");
      this.$tagsHolder.removeClass("active");
      const $active = this.$tags.filter(".active");
      if ($active.length) {
        const activeOffset = $active.offset();
        const parentOffset = $active.parent().offset();
        const tagsOuterWidth = this.$tagsOuter.width();
        const activeWidth = $active.outerWidth();
        if (activeOffset && parentOffset && tagsOuterWidth && activeWidth) {
          const offset = activeOffset.left - parentOffset.left;
          const scrollLeft = -(tagsOuterWidth - activeWidth - offset);
          this.$tagsHolder.css({
            width: $active.outerWidth() || 0,
            marginRight: 10
          }).animate({
            scrollLeft: scrollLeft
          }, 400, function () {
            $(this).css({ "overflow": "hidden" });
          });
        }
      }
      else {
        this.clearTagHolderCss();
      }
    }
  }
}
import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

/**
 * Search related events
 *   search.text.change(text: string)
 *   search.clear()
 *   search.submit(text: string)
 *   search.result.fetched(result: MangaData[])
 *   search.result.failed()
 *   search.result.opend()
 *   search.result.close()
 *   search.result.closed()
 */

type SearchBarState = {
  text: string,
  focused: boolean,
  loading: boolean,
  clearShowing: boolean,
};

export default class SearchResult extends Controller<SearchBarState> {
  $form: JQuery<HTMLElement>;
  $input: JQuery<HTMLInputElement>;
  $loading: JQuery<HTMLElement>;
  $clear: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    // Initialize elements
    this.$form = root;
    this.$input = root.find("#index-search") as JQuery<HTMLInputElement>;
    this.$loading = root.find("#index-search-loading");
    this.$clear = root.find("#index-search-clear");

    // Initialize events
    this.$input.focus(() => this.focus());

    this.$input.blur(() => this.blur());

    this.$input.on('input', () => this.change());

    this.$input.keyup((e) => {
      if (e.key === "Escape") EventPool.emit("search.result.close");
    });

    this.$clear.click(() => {
      EventPool.emit("search.result.close");
    });

    this.$form.click(() => {
      this.focus();
      this.$input.focus();
    });

    this.$form.submit(() => {
      this.submit();
      return false;
    });

    this.listen("search.result.closed", () => {
      this.setState({ text: "", clearShowing: false, loading: false });
    });

    this.listen("search.result.fetched", () => {
      this.setState({ loading: false });
    });

    this.listen("search.result.opened", () => {
      this.setState({ clearShowing: true });
    });
  }

  initialState() : SearchBarState {
    return { text: "", focused: false, loading: false, clearShowing: false };
  }

  focus() {
    this.setState({ focused: true });
  }

  blur() {
    const { text } = this.state;
    if (text.trim() === '') {
      EventPool.emit("search.result.close");
      this.setState({ focused: false, clearShowing: false });
    }
  }

  change() {
    this.setState({ text: this.$input.val() as string });
  }

  submit() {
    this.setState({ loading: true, clearShowing: false });
    const val = this.state.text.trim();
    if (val && val.length > 0) {
      EventPool.emit("search.submit", val);
    } else {
      EventPool.emit("search.result.close");
      this.setState({ loading: false });
    }
  }

  update() {
    const { text, focused, loading, clearShowing } = this.state;

    this.$input.val(text);

    if (focused) this.$form.addClass("focus");
    else this.$form.removeClass("focus");

    if (loading) this.$loading.fadeIn('fast');
    else this.$loading.fadeOut('fast');

    if (clearShowing) this.$clear.fadeIn('fast');
    else this.$clear.fadeOut('fast');
  }
}
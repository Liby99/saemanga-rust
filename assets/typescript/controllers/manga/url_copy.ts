import ClipBoardJs from 'clipboard';

import Controller from '../../library/controller';

export default class UrlCopy extends Controller<{}> {

  static DURATION : number = 2000;

  copiedShowing: boolean;
  clipboard: any;

  $tag: JQuery<HTMLElement>;

  constructor(root: JQuery<HTMLElement>) {
    super(root);

    this.clipboard = new ClipBoardJs(`#share-url-button`);
    this.copiedShowing = false;

    this.$tag = this.root.children("#copied-tag");

    this.clipboard.on("success", (event: any) => {
      if (!this.copiedShowing) {
        this.copiedShowing = true;
        this.$tag.removeClass("hidden");
        setTimeout(() => {
          this.copiedShowing = false;
          this.$tag.addClass("hidden");
        }, UrlCopy.DURATION);
      }
      event.clearSelection();
    });
  }

  initialState() : {} {
    return {};
  }
}
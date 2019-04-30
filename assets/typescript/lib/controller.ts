import $ from "jquery";
import Component from "./component";

interface ComponentClass<T> {
  new (root: JQuery<HTMLElement>): Component<T>;
}

export default class Controller {
  registry: { [key: string]: ComponentClass<any> } = {};

  register<T>(name: string, comp: ComponentClass<T>) {
    this.registry[name] = comp;
  }

  build() {
    $("body").find("[component]").each((index, element) => {
      const $elem = $(element);
      const name = $elem.attr("component");
      if (name !== undefined) {
        const c = this.registry[name];
        if (c) {
          new c($elem);
        } else {
          throw new Error(`Unknown Component [${name}]`);
        }
      } else {
        console.error("Unknown error: doesn't have component attr");
      }
    });
  }
}
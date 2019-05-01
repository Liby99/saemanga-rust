import $ from "jquery";
import Component from "./component";

interface ComponentClass<T> {
  new (root: JQuery<HTMLElement>): Component<T>;
}

export default class Registry {
  static registry: { [key: string]: ComponentClass<any> } = {};

  static register<T>(name: string, comp: ComponentClass<T>) {
    if (!this.registry[name]) {
      this.registry[name] = comp;
    } else {
      throw new Error(`Component ${name} already registered`);
    }
  }

  static build() {
    $("[component]").each((index, element) => {
      const $elem = $(element);
      const name = $elem.attr("component");
      if (name !== undefined) {
        const c = this.registry[name];
        if (c) {
          new c($elem);
        } else {
          console.error(`Unknown Component [${name}]`);
        }
      } else {
        console.error("Unknown error: doesn't have component attr");
      }
    });
  }
}
import $ from "jquery";
import Controller from "./controller";

interface ControllerClass<T> {
  new (root: JQuery<HTMLElement>): Controller<T>;
}

export default class Registry {

  static ATTR: string = "ctrl";

  static registry: { [key: string]: ControllerClass<any> } = {};

  /**
   * Register a controller to be used by html
   * @param name Registered name of the controller. Can be referred to in html using `controller="[name]"`
   * @param comp The Controller class. Need to be extending Controller
   */
  static register<T>(name: string, comp: ControllerClass<T>) {
    if (!this.registry[name]) {
      this.registry[name] = comp;
    } else {
      throw new Error(`Component ${name} already registered`);
    }
  }

  static build() {
    $(`[${this.ATTR}]`).each((_, element) => this.buildController(element));
  }

  static buildWithRoot(element: JQuery<HTMLElement>) {
    element.find(`[${this.ATTR}]`).each((_, element) => this.buildController(element));
  }

  static buildController(element: HTMLElement) {
    const $elem = $(element);
    const name = $elem.attr(this.ATTR);
    if (name !== undefined) {
      const c = this.registry[name];
      if (c) {
        new c($elem);
      } else {
        console.error(`Unknown controller [${name}]`);
      }
    } else {
      console.error("Unknown error: element doesn't have controller attr");
    }
  }
}
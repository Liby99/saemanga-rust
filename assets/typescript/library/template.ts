import $ from 'jquery';
import Handlebars from 'handlebars';
import Registry from './registry';

export default class Template<Data> {

  template: Handlebars.TemplateDelegate<Data>;

  constructor(template: Handlebars.TemplateDelegate<Data>) {
    this.template = template;
  }

  render(data: Data | Data[]) : string {
    if (Array.isArray(data)) {
      return data.map(d => this.template(d)).join();
    } else {
      return this.template(data);
    }
  }

  /**
   * Mount the rendered data to element
   */
  mountTo(data: Data | Data[], root: JQuery<HTMLElement>) {
    root.html(""); // Clear the root
    this.appendTo(data, root); // Append to root
  }

  /**
   * Append the rendered data to element
   */
  appendTo(data: Data | Data[], root: JQuery<HTMLElement>) {
    const elem = $(this.render(data));
    Registry.buildWithRoot(elem);
    elem.appendTo(root);
  }
}
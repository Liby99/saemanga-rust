import Handlebars from 'handlebars';

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
  mount(data: Data | Data[], elem: JQuery<HTMLElement>) {
    elem.html(this.render(data));
  }
}
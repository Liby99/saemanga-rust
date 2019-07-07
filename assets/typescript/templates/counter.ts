import hbs from '../../templates/tests/front_end/counter.html.hbs';
import Template from '../library/template';

export class CounterTemplate extends Template<{}> {
  constructor() {
    super(hbs);
  }
}

export default new CounterTemplate();
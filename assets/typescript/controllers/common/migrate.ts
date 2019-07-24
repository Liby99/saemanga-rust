import Cookie from 'cookie_js';
import { default as Axios, AxiosResponse } from 'axios';

import Controller from '../../library/controller';
import EventPool from '../../library/event_pool';

type CanMigrateResponse = {
  can: boolean
}

export default class Migrate extends Controller<{}> {
  constructor(root: JQuery<HTMLElement>) {
    super(root);

    setTimeout(() => {

      // First check if there's username
      const username = Cookie.get("username");
      if (username) {

        // Then go to server and check if we can do migration
        Axios.post("/ajax/user/can_migrate").then((response: AxiosResponse<CanMigrateResponse>) => {
          const { can } = response.data;
          if (can) {
            EventPool.emit("panel.migrate.open");
          }
        })
      }
    }, 1000);
  }

  initialState() : {} {
    return {};
  }
}
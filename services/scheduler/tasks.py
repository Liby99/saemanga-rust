from typing import Dict, List, Callable

import datetime
import requests

def task(name, cmd, interval):
  """
  Task constructor.
  @param name: the name being printed in the log
  @param cmd: the cmd to send to `/admin/` directory
  @param interval: a number representing the time interval of the task
  """

  def action(conf, jar):
    """
    Action function. Sending a signal to server admin
    """
    print(f"[scheduler] Running scheduled task {name} [{datetime.datetime.now()}]")
    fetch_url = f"http://{conf['addr']}:{conf['port']}/admin/{cmd}"
    requests.post(url=fetch_url, cookies=jar)

  return {
    "action": action,
    "interval": interval
  }

def tasks():
  return [

    # Fetching overall latest manga, per 30 minute
    task("fetch_overall_latest_manga", "latest/fetch_overall", 1000 * 60 * 30),

    # Fetching latest manga of genres, per 4 hours
    task("fetch_latest_manga_of_genres", "latest/fetch_genres", 1000 * 60 * 60 * 4),

    # Fetching the oldest updating 50 manga, per 1 hour
    task("fetch_oldest_updating_50", "latest/fetch_oldest_updating", 1000 * 60 * 60),

    # Purge the expired user session, per 1 day
    task("purge_expired_session", "user/session/purge", 1000 * 60 * 60 * 24)
  ]
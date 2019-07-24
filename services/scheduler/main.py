from typing import Dict, List, Callable
import sched, time, sys, requests, datetime

from account import account
from config import config

DELAY = False
PROD = False

def tasks():
  return [

    # Fetching overall latest manga, per 30 minute
    task("fetch_overall_latest_manga", "latest/fetch_overall", 60 * 30),

    # Fetching latest manga of genres, per 4 hours
    task("fetch_latest_manga_of_genres", "latest/fetch_genres", 60 * 60 * 4),

    # Fetching the oldest updating 50 manga, per 1 hour
    task("fetch_oldest_updating_50", "latest/fetch_oldest_updating", 60 * 60),

    # Purge the expired user session, per 1 day
    task("purge_expired_session", "user/session/purge", 60 * 60 * 24)
  ]

def task(name, cmd, interval):
  def action(conf, jar):
    print(f"[scheduler] Running scheduled task {name} [{datetime.datetime.now()}]")
    fetch_url = f"http://{conf['addr']}:{conf['port']}/admin/{cmd}"
    try:
      requests.post(url=fetch_url, cookies=jar)
    except Exception as err:
      print(err)

  return {
    "action": action,
    "interval": interval
  }

def parse_args():
  global DELAY
  global PROD
  args = sys.argv[1:]
  for arg in args:
    if arg == "--delay":
      DELAY = True
    if arg == "--prod":
      PROD = True

def login(conf, acct):
  url = f"http://{conf['addr']}:{conf['port']}/user/login"
  login_response = requests.post(url=url, data=acct)
  return login_response.cookies["session"]

def periodic(scheduler, interval, action, actionargs=()):
  scheduler.enter(interval, 1, periodic, (scheduler, interval, action, actionargs))
  action(*actionargs)

def main():
  conf = config(PROD)

  session = login(conf, account)
  jar = requests.cookies.RequestsCookieJar()
  jar.set("session", session, path="/")

  scheduler = sched.scheduler(time.time, time.sleep)

  for task in tasks():
    periodic(scheduler, task["interval"], task["action"], (conf, jar))

  scheduler.run()

if __name__ == "__main__":
  parse_args()
  if DELAY:
    time.sleep(1)
  main()
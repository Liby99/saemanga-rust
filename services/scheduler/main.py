"""
The main entry point of scheduler.
Usage:

> python3 main.py [--delay] [--prod]

Parameters:

--delay: delay 1000 seconds after launch. Good for prod env waiting for server
--prod: use production host and port
"""

import sched
import time
import sys
import requests

from account import account
from config import config
from tasks import tasks

DELAY = False
PROD = False

def parse_args():
  """
  Parse the command line arguments
  """
  global DELAY
  global PROD
  args = sys.argv[1:]
  for arg in args:
    if arg == "--delay":
      DELAY = True
    if arg == "--prod":
      PROD = True

def login(conf, acct):
  """
  Login the website with config and get the session id with the acctount info
  """
  url = f"http://{conf['addr']}:{conf['port']}/user/login"
  login_response = requests.post(url=url, data=acct)
  return login_response.cookies["session"]

def periodic(scheduler, interval, action, actionargs=()):
  """
  Do periodic action
  """
  scheduler.enter(interval, 1, periodic, (scheduler, interval, action, actionargs))
  action(*actionargs)

def main():
  """
  Main entry point
  """
  conf = config(PROD)
  acct = account()

  session = login(conf, acct)
  jar = requests.cookies.RequestsCookieJar()
  jar.set("session", session, path="/")

  scheduler = sched.scheduler(time.time, time.sleep)

  for task in tasks():
    periodic(scheduler, task["interval"], task["action"], (conf, jar))

  scheduler.run()

if __name__ == "__main__":
  parse_args()
  if DELAY:
    time.sleep(1000)
  main()
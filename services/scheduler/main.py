import requests
import sched
import time

from account import account
from config import config
from tasks import tasks

def login(conf, acct):
  url = f"http://{conf['addr']}:{conf['port']}/user/login"
  login_response = requests.post(url=url, data=acct)
  return login_response.cookies["session"]

def periodic(scheduler, interval, action, actionargs=()):
  scheduler.enter(interval, 1, periodic, (scheduler, interval, action, actionargs))
  action(*actionargs)

def main():
  conf = config(False)
  acct = account()

  session = login(conf, acct)
  jar = requests.cookies.RequestsCookieJar()
  jar.set("session", session, path="/")

  scheduler = sched.scheduler(time.time, time.sleep)

  for task in tasks():
    periodic(scheduler, task["interval"], task["action"], (conf, jar))

  scheduler.run()

if __name__ == "__main__":
  main()
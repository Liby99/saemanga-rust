import requests
import sched
import time

import account
import config
import tasks

def login(conf, acct):
  url = "http://{}:{}/user/login".format(conf["addr"], conf["port"])
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

  for task_fn in tasks():
    task = task_fn()
    periodic(scheduler, task["interval"], task["action"], (conf, jar))

  scheduler.run()

if __name__ == "__main__":
  main()
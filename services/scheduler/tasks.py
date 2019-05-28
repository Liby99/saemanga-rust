from typing import Dict, List, Callable

import datetime
import requests

def tasks():
  return [
    fetch_overall_latest_manga,
    fetch_latest_manga_of_genres,
    fetch_oldest_50_manga,
    clear_session
  ]

def print_task(name):
  print("Running scheduled task {} [{}]".format(name, datetime.datetime.now()))

def fetch_overall_latest_manga():
  def action(conf, jar):
    print_task("fetch_overall_latest_manga")
    fetch_url = "http://{}:{}/admin/latest/fetch_overall".format(conf["addr"], conf["port"])
    requests.post(url=fetch_url, cookies=jar)

  return {
    "action": action,
    "interval": 1000 * 60 * 30 # Per Half an hour
  }

def fetch_latest_manga_of_genres():
  def action(conf, jar):
    print_task("fetch_latest_manga_of_genres")
    fetch_url = "http://{}:{}/admin/latest/fetch_genres".format(conf["addr"], conf["port"])
    requests.post(url=fetch_url, cookies=jar)

  return {
    "action": action,
    "interval": 1000 * 60 * 60 * 4 # Per 4 Hours
  }

def fetch_oldest_50_manga():
  def action(conf, jar):
    print_task("fetch_oldest_50_manga")
    print("TODO: Not implemented")

  return {
    "action": action,
    "interval": 1000 * 60 * 60 # Per Hour
  }

def clear_session():
  def action(conf, jar):
    print_task("clear_session")
    print("TODO: Not implemented")

  return {
    "action": action,
    "interval": 1000 * 60 * 60 * 24 # Per day
  }
import pymongo
import subprocess
import json
import sys
import datetime

from mongo_account import conn
from temp_password import temp_password

# Print iterations progress
def print_progress(iteration, total, prefix = '', suffix = '', decimals = 1, length = 60, fill = '█'):
  percent = ("{0:." + str(decimals) + "f}").format(100 * (iteration / float(total)))
  filledLength = int(length * iteration // total)
  bar = fill * filledLength + '-' * (length - filledLength)
  print('\r%s |%s| %s%% %s' % (prefix, bar, percent, suffix), end = '\r')
  if iteration == total:
    print()

def main():
  cli = pymongo.MongoClient(conn["host"])
  old_db = cli[conn["old_database"]]
  new_db = cli[conn["new_database"]]

  user_only = False
  manga_only = False
  for arg in sys.argv[1:]:
    if arg == "--user-only":
      user_only = True
    elif arg == "--manga-only":
      manga_only = True

  if user_only:
    migrate_users(old_db, new_db) # old_user._id -> new_user._id
  elif manga_only:
    migrate_mangas(old_db, new_db) # old_manga._id -> { new_manga._id, new_manga.dmk_id }
  else:
    user_migrates = migrate_users(old_db, new_db) # old_user._id -> new_user._id
    manga_migrates = migrate_mangas(old_db, new_db) # old_manga._id -> { new_manga._id, new_manga.dmk_id }
    # follow_migrates = migrate_follows(???)

def migrate_users(old_db, new_db):
  old_user_coll = old_db["user"]
  new_user_coll = new_db["user"]
  old_users = old_user_coll.find()
  user_migrates = {}
  total_user_count = old_user_coll.count_documents({})
  completed_count = 0
  skipped_count = 0
  for old_user in old_users:
    result = migrate_user(old_user, new_user_coll)
    new_user_id = result["id"]
    user_migrates[str(old_user["_id"])] = new_user_id
    completed_count += 1
    if result["existed"]:
      skipped_count += 1
    print_progress(completed_count, total_user_count, prefix="Migrating users...")
  print(f"Total amount: {completed_count}, Skipped count: {skipped_count}")
  return user_migrates

def migrate_user(old_user, new_user_coll):
  existed = new_user_coll.find_one({ "username": old_user["username" ]})
  if existed:
    # print(f"New user {old_user['username']} already existed")
    return { "id": existed['_id'], "existed": True }
  else:
    new_user = {
      "username": old_user["username"],
      "password": encode_password(temp_password),
      "is_admin": False,
      "visit_count": old_user["visit_amount"],
      "register_date_time": old_user["register_date_time"],
      "last_visit_date_time": old_user["last_visit"]
    }
    result = new_user_coll.insert_one(new_user)
    return { "id": result.inserted_id, "existed": False }

def encode_password(password):
  output = subprocess.check_output(["cargo", "run", "-q", "--bin", "enc_password", password])
  return output.decode()

def migrate_mangas(old_db, new_db):
  old_manga_coll = old_db["manga"]
  new_manga_coll = new_db["manga"]
  old_mangas = old_manga_coll.find()
  total_manga_count = old_manga_coll.count_documents({})
  completed_count = 0
  skipped_count = 0
  failed_dmk_ids = []
  manga_migrates = {}
  for old_manga in old_mangas:
    result = migrate_manga(old_manga, new_manga_coll)
    completed_count += 1
    if "failed" in result:
      failed_dmk_ids.append(old_manga["dmk_id"])
    else:
      if result["existed"]:
        skipped_count += 1
      manga_migrates[old_manga["_id"]] = result
    print_progress(completed_count, total_manga_count, prefix="Migrating mangas...")
  failed_count = len(failed_dmk_ids)
  print(f"Completed count: {completed_count - failed_count}, Skipped count: {skipped_count}")
  print(failed_dmk_ids)

def migrate_manga(old_manga, new_manga_coll):
  dmk_id = old_manga["dmk_id"]
  new_manga = new_manga_coll.find_one({ "dmk_id": dmk_id })
  if new_manga:
    # print(f"Manga {dmk_id} already existed")
    return { "_id": new_manga["_id"], "dmk_id": dmk_id, "existed": True, "failed": False }
  else:
    try:
      new_manga = fetch_new_manga(old_manga)
      result = new_manga_coll.insert_one(new_manga)
      return { "_id": result.inserted_id, "dmk_id": dmk_id, "existed": False, "failed": False }
    except:
      return { "failed": True }

def fetch_new_manga(old_manga):
  dmk_id = old_manga["dmk_id"]
  output = subprocess.check_output(["cargo", "run", "-q", "--bin", "get_manga_info", dmk_id])
  manga = json.loads(output.decode())
  manga.pop("_id", None)
  manga["add_date_time"] = old_manga["insert_date"]
  manga["update_date_time"] = datetime.datetime.utcnow()
  manga["refresh_date_time"] = datetime.datetime.utcnow()
  return manga

if __name__ == "__main__":
  main()
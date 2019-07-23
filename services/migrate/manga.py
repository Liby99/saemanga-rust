import datetime
import subprocess
import json

from util import print_progress

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
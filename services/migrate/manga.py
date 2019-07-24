import datetime, subprocess, threading, json, math

from async_call import Async
from util import print_progress

def migrate_mangas(old_db, new_db):

  old_manga_coll = old_db["manga"]
  new_manga_coll = new_db["manga"]
  old_mangas = old_manga_coll.find()
  total_manga_count = old_manga_coll.count_documents({})
  migrated_count = 0
  processed_count = 0
  skipped_count = 0
  failed_dmk_ids = []
  manga_migrates = {}

  # Generate the batches
  batch_size = 50
  batches = []
  count = 0
  for old_manga in old_mangas:
    if count == 0:
      batches.append([])
    batches[-1].append(old_manga)
    count += 1
    if count == batch_size:
      count = 0

  # Fetch in batches
  for batch in batches:

    # Generate the threads
    threads = [] # List[Tuple[Manga, Thread]]
    for old_manga in batch:
      thread = migrate_manga(old_manga, new_manga_coll)
      threads.append((old_manga, thread))
      # print_progress(len(threads), total_manga_count, prefix="Spawning threads...")

    # Join all the threads and get the result
    for (old_manga, thread) in threads:
      result = thread.wait()
      processed_count += 1
      if result["failed"]:
        failed_dmk_ids.append(old_manga["dmk_id"])
      else:
        if result["existed"]:
          skipped_count += 1
        else:
          migrated_count += 1
        manga_migrates[old_manga["_id"]] = result
      print_progress(processed_count, total_manga_count, prefix="Migrating mangas...")

  failed_count = len(failed_dmk_ids)
  print(f"Total: {total_manga_count}, Completed: {migrated_count}, Skipped: {skipped_count}, Failed: {failed_count}")
  if failed_count > 0:
    print(f"Failed dmk ids: {failed_dmk_ids}")
  return manga_migrates

@Async
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
    except Exception as err:
      return { "dmk_id": dmk_id, "failed": True }

def fetch_new_manga(old_manga):
  dmk_id = old_manga["dmk_id"]
  output = subprocess.check_output(["cargo", "run", "-q", "--bin", "get_manga_info", dmk_id])
  manga = json.loads(output.decode())
  manga.pop("_id", None)
  manga["add_date_time"] = old_manga["insert_date"]
  manga["update_date_time"] = datetime.datetime.utcnow()
  manga["refresh_date_time"] = datetime.datetime.utcnow()
  return manga
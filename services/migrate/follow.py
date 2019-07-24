from util import print_progress
from bson.objectid import ObjectId

def migrate_follows(old_db, new_db, user_migrates, manga_migrates):
  """
  user_migrates: { [old_user_id]: [new_user_id] }
  manga_migrates: { [old_manga_id]: { "_id": [new_manga_id], "dmk_id": [dmk_id] } }
  """

  print(">>> Migrating Follows")

  old_follow_coll = old_db["follow"]
  new_follow_coll = new_db["follow"]
  old_follows = old_follow_coll.find()

  total_follow_count = old_follow_coll.count_documents({})
  migrated_count = 0
  processed_count = 0
  skipped_count = 0

  failed_follows = []

  print(manga_migrates)

  for old_follow in old_follows:
    old_user_id = old_follow["user_id"]
    old_manga_id = old_follow["manga_id"]
    if old_user_id in user_migrates:
      new_user_id = user_migrates[old_user_id]
      if old_manga_id in manga_migrates:
        manga_dmk_id = manga_migrates[old_manga_id]["dmk_id"]

        # Check if the follow has already existed
        new_follow = new_follow_coll.find_one({
          "user_id": ObjectId(new_user_id),
          "manga_dmk_id": manga_dmk_id
        })

        if new_follow:
          skipped_count += 1
        else:
          # Is liked information
          if "liked" in old_follow:
            is_liked = old_follow["liked"]
          else:
            is_liked = False

          # Generate a new follow document
          new_follow = {
            "user_id": ObjectId(new_user_id),
            "manga_dmk_id": manga_dmk_id,
            "start_date_time": old_follow["start_date"],
            "update_date_time": old_follow["update_date"],
            "current_episode": old_follow["current_episode"],
            "max_episode": old_follow["max_episode"],
            "is_up_to_date": old_follow["up_to_date"],
            "is_liked": is_liked
          }

          # Insert into the new collection
          new_follow_coll.insert_one(new_follow)
          migrated_count += 1
      else:
        failed_follows.append({
          "old_user_id": old_user_id,
          "old_manga_id": old_manga_id,
          "reason": "Manga not found"
        })
    else:
      failed_follows.append({
        "old_user_id": old_user_id,
        "old_manga_id": old_manga_id,
        "reason": "User not found"
      })

    processed_count += 1
    print_progress(processed_count, total_follow_count, prefix="Migrating follow...")

  failed_count = len(failed_follows)
  print(f"Total: {total_follow_count}, Migrated: {migrated_count}, Skipped: {skipped_count}, Failed: {failed_count}")
  # if failed_count > 0:
  #   print(f"Failed follows: {failed_follows}")
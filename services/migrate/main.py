import pymongo
import sys

from mongo_account import conn
from user import migrate_users
from manga import migrate_mangas

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
    # user_migrates = migrate_users(old_db, new_db) # old_user._id -> new_user._id
    # manga_migrates = migrate_mangas(old_db, new_db) # old_manga._id -> { new_manga._id, new_manga.dmk_id }
    # follow_migrates = migrate_follows(???)
    pass

if __name__ == "__main__":
  main()
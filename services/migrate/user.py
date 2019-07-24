import subprocess
from bson.objectid import ObjectId

from temp_password import temp_password
from util import print_progress

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
    new_user_id = result["_id"]
    user_migrates[old_user["_id"]] = new_user_id
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
    return { "_id": existed['_id'], "existed": True }
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
    return { "_id": ObjectId(result.inserted_id), "existed": False }

def encode_password(password):
  output = subprocess.check_output(["cargo", "run", "-q", "--bin", "enc_password", password])
  return output.decode()
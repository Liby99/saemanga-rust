# Database Migration

Need to migrate from original `manga` database to `saemanga` database.

Need to migrate mainly user data, follow data and manga data.

Need to be incremental. e.g. If new user follows new manga we should insert the new one and update the original one.

## Initialize Connections

```
cp temp_password_dummy.py temp_password.py
cp mongo_account_dummy.py mongo_account.py
```

## User data migration

Since the ID will change, we need to use the username data as ID. We have also changed the password algorithm so a temporary password will be created.
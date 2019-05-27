# User

``` json
{
  "_id": "<ObjectId>, Unique",
  "username": "<string>, Unique",
  "password": "<string>",
  "is_admin": "<boolean>",
  "visit_count": "<int>",
  "register_date_time": "<Date>",
  "last_login_date_time": "<Date>"
}
```

# Session

``` json
{
  "_id": "<ObjectId>, Unique",
  "user_id": "<ObjectId>",
  "start_date_time": "<Date>",
  "last_login_date_time": "<Date>",
  "expire_date_time": "<Date>"
}
```

# Manga

``` json
{
  "_id": "<ObjectId>",
  "add_date_time": "<Date>",
  "update_date_time": "<Date>",
  "refresh_date_time": "<Date>",
  "dmk_id": "<string>",
  "dmk_id_base": {
    "version": "<string>",
    "dmk_id_web": "<string>",
    "dmk_id_home": "<string>"
  },
  "title": "<string>",
  "description": "<string>",
  "author": "<string>",
  "tags": ["<string>"],
  "genre": "<string>",
  "status": "<string>, 'updating' | 'ended'",
  "episodes": [{
    "index": "<int>",
    "is_book": "<boolean>",
    "episode": "<int>",
    "num_pages": "<int>",
    "dmk_directory": "<string>"
  }]
}
```

# Follow

``` json
{
  "_id": "<ObjectId>",
  "user_id": "<ObjectId>, Index",
  "manga_dmk_id": "<ObjectId>, Index",
  "start_date_time": "<Date>",
  "update_date_time": "<Date>",
  "current_episode": "<int>",
  "max_episode": "<int>",
  "is_up_to_date": "<boolean>"
}
```
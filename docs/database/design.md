# User

| Field | Type | Description | Unique |
|-------|------|-------------|--------|
| `_id` | `ObjectId` | The object id | `true` |
| `username` | `String` | Username | `true` |
| `password` | `String` | Password | |
| `register_time` | `DateTime` | Register Date Time | |
| `login_count` | `int` | The number this user has logged in | |
| `last_login_time` | `DateTime` | Last Login TIme | |
| `is_admin` | `bool` | Is administrator flag | |

# Session

| Field | Type | Description | Unique |
|-------|------|-------------|--------|
| `_id` | `ObjectId` | Session Id | `true` |
| `user_id` | `ObjectId` | User Id | |
| `begin_time` | `DateTime` | The time that this session has starts | |
| `expire_time` | `DateTime` | The time that this session should end | |
| `touch_count` | `int` | The number of time that this session has been touched | |

# Manga

| Field | Type | Description | Unique |
|-------|------|-------------|--------|
| `_id` | `ObjectId` | Internal Manga Id | `true` |
| `dmk_id` | `String` | Four digit id from Cartoonmad | `true` |
| `dmk_info.version` | `int` | Integer representing the version | |
| `dmk_info.id_gen` | `String` | | |
| `dmk_info.id_home` | `String` | | |
| `dmk_info.id_web` | `String` | | |
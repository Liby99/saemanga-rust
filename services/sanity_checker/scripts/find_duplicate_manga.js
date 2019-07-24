db.manga.aggregate([
  {
    "$group": {
      "_id": {
        "dmk_id": "$dmk_id"
      },
      "uniqueIds": {
        "$addToSet": "$_id"
      },
      "count": {
        "$sum": 1
      }
    }
  },
  {
    "$match": {
      count: {
        "$gt": 1
      }
    }
  },
  {
    "$sort": {
      count: -1
    }
  }
]);
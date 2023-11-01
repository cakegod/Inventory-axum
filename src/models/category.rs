use mongodb::bson::oid::ObjectId;

struct Category {
    name: String,
    description: String,
    url: String,
    _id: ObjectId,
}

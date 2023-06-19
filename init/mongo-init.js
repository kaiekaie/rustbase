print(
  "Start #################################################################"
);
db = connect("localhost:27017/rustplatform");

db.createCollection("users", {
  validator: {
    $jsonSchema: {
      required: ["_id", "username", "role", "created"],
      properties: {
        _id: {
          bsonType: "objectId",
        },
        username: {
          bsonType: "string",
          pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
        },
        name: {
          bsonType: ["null", "date"],
        },
        created: {
          bsonType: "date",
        },
        modified: {
          bsonType: ["null", "date"],
        },
        role: {
          enum: ["Admin", "User"],
          bsonType: "string",
        },
      },
    },
  },
});
db.createCollection("admins", {
  validator: {
    $jsonSchema: {
      required: ["_id", "username", "created"],
      properties: {
        _id: {
          bsonType: "objectId",
        },
        username: {
          bsonType: "string",
          pattern: "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
        },
        name: {
          bsonType: ["null", "string"],
        },
        created: {
          bsonType: "date",
        },
        modified: {
          bsonType: ["null", "date"],
        },
      },
    },
  },
});

db.createCollection("secrets", {
  validator: {
    $jsonSchema: {
      required: ["_id", "created", "hash", "user_id"],
      properties: {
        _id: {
          bsonType: "objectId",
        },
        created: {
          bsonType: "date",
        },
        modified: {
          bsonType: ["null", "date"],
        },
        hash: {
          bsonType: "string",
        },
        user_id: {
          bsonType: ["objectId", "null"],
        },
      },
    },
  },
});

db.createCollection("collections", {
  validator: {
    $jsonSchema: {
      required: ["_id", "name"],
      properties: {
        _id: {
          bsonType: "objectId",
        },
        name: {
          bsonType: "string",
        },
        created: {
          bsonType: "date",
        },
        modified: {
          bsonType: ["null", "date"],
        },
      },
    },
  },
});

db.createRole({
  role: "noSecretsAccess",
  privileges: [
    {
      resource: { db: "rustplatform", collection: "" },
      actions: ["find", "insert", "update", "remove"],
    },
    {
      resource: { db: "rustplatform", collection: "secrets" },
      actions: ["find", "insert", "update", "remove"],
    },
  ],
  roles: [],
});

db.createUser({
  user: "admin",
  pwd: "yourpassword",
  roles: ["noSecretsAccess"],
});

print("End #################################################################");

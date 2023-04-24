print(
  "Start #################################################################"
);
db = connect("localhost:27017/rustplatform");
db.createCollection("documents");

db.createRole({
  role: "noSecretsAccess",
  privileges: [
    {
      resource: { db: "documents", collection: "" },
      actions: ["find", "insert", "update", "remove"],
    },
    {
      resource: { db: "documents", collection: "secrets" },
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

print(
  "Start #################################################################"
);
db = connect("localhost:27017/rustplatform");
db.createCollection("documents");

db.createUser({
  user: "admin",

  pwd: "yourpassword",

  roles: [{ role: "readWrite", db: "documents" }],
});

print("End #################################################################");

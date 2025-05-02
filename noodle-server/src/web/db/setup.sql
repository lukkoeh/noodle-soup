DROP TABLE IF EXISTS "user";
DROP TABLE IF EXISTS "role";

CREATE TABLE IF NOT EXISTS "user" (
    "id" BIGSERIAL PRIMARY KEY,
    "firstname" VARCHAR(255),
    "lastname" VARCHAR(255),
    "email" VARCHAR(255) UNIQUE,
    "password" CHARACTER(60)
);
CREATE INDEX ON "user" ("firstname");
CREATE INDEX ON "user" ("lastname");
CREATE INDEX ON "user" ("email");

CREATE TABLE IF NOT EXISTS "role" (
    "id" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR(32) UNIQUE,
    "permissions" json
)

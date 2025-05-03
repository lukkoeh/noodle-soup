DROP TABLE IF EXISTS "user";
DROP TABLE IF EXISTS "role";
DROP TABLE IF EXISTS "group";
DROP TABLE IF EXISTS "user_has_role";
DROP TABLE IF EXISTS "user_in_group";
DROP TABLE IF EXISTS "user_has_permissions";
DROP TYPE IF EXISTS "group_kind";

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
);

CREATE TYPE "group_kind" AS ENUM ('organization', 'learning', 'contact');
CREATE TABLE IF NOT EXISTS "group" (
    "id" BIGSERIAL PRIMARY KEY,
    "kind" "group_kind",
    "name" VARCHAR(255) UNIQUE,
    "parent" BIGINT REFERENCES "group"(id)
);

CREATE TABLE IF NOT EXISTS "user_has_role" (
    "user_id" BIGSERIAL REFERENCES "user",
    "role_id" BIGSERIAL REFERENCES "role"
);

CREATE TABLE IF NOT EXISTS "user_in_group" (
    "user_id" BIGSERIAL REFERENCES "user",
    "group_id" BIGSERIAL REFERENCES "group"
);

CREATE TABLE IF NOT EXISTS "user_has_permissions" (
    "user_id" BIGSERIAL REFERENCES "user",
    "resource" VARCHAR(255) NOT NULL,
    "permission" SMALLINT DEFAULT 0
);

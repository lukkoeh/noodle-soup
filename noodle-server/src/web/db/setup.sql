DROP TABLE IF EXISTS "user_has_permissions";
DROP TABLE IF EXISTS "user_has_role";
DROP TABLE IF EXISTS "role";
DROP TABLE IF EXISTS "user_in_group";
DROP TABLE IF EXISTS "group";
DROP TABLE IF EXISTS "user";
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
    "parent" BIGINT REFERENCES "group"(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS "user_has_role" (
    "user_id" BIGSERIAL REFERENCES "user" ON DELETE CASCADE,
    "role_id" BIGSERIAL REFERENCES "role" ON DELETE CASCADE,
    PRIMARY KEY ("user_id", "role_id")
);

CREATE TABLE IF NOT EXISTS "user_in_group" (
    "user_id" BIGSERIAL REFERENCES "user" ON DELETE CASCADE,
    "group_id" BIGSERIAL REFERENCES "group" ON DELETE CASCADE,
    PRIMARY KEY ("user_id", "group_id")
);

CREATE TABLE IF NOT EXISTS "user_has_permissions" (
    "user_id" BIGSERIAL REFERENCES "user" ON DELETE CASCADE,
    "resource" VARCHAR(255) NOT NULL,
    "permission" SMALLINT DEFAULT 0
);

DROP TABLE IF EXISTS "file" CASCADE;

DROP TABLE IF EXISTS "user_permissions" CASCADE;

DROP TABLE IF EXISTS "role_permissions" CASCADE;

DROP TABLE IF EXISTS "group_permissions" CASCADE;

DROP TABLE IF EXISTS "user_has_role" CASCADE;

DROP TABLE IF EXISTS "role" CASCADE;

DROP TABLE IF EXISTS "user_in_group" CASCADE;

DROP TABLE IF EXISTS "group" CASCADE;

DROP TABLE IF EXISTS "user" CASCADE;

DROP TYPE IF EXISTS "group_kind" CASCADE;

DROP TABLE IF EXISTS "course" CASCADE;

DROP TABLE IF EXISTS "content_section" CASCADE;

DROP TABLE IF EXISTS "content_element" CASCADE;

DROP TABLE IF EXISTS "file_in_content_element" CASCADE;

DROP TABLE IF EXISTS "template" CASCADE;

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

CREATE TYPE "group_kind" AS ENUM ('organization', 'learning', 'contact', 'role');

CREATE TABLE IF NOT EXISTS "group" (
    "id" BIGSERIAL PRIMARY KEY,
    "kind" "group_kind",
    "name" VARCHAR(255) UNIQUE,
    "parent" BIGINT REFERENCES "group" (id) ON DELETE SET NULL
);

CREATE INDEX ON "group" ("kind");

CREATE TABLE IF NOT EXISTS "role" (
    "id" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR(32) UNIQUE,
    "group" BIGINT REFERENCES "group" ON DELETE SET NULL DEFAULT NULL,
    "permissions" json
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

CREATE TABLE IF NOT EXISTS "user_permissions" ( -- `user` -> CRUD rights for `user`
    "user_id" BIGSERIAL REFERENCES "user" ON DELETE CASCADE,
    "resource_id" BIGINT REFERENCES "user" ON DELETE CASCADE DEFAULT NULL,
    "permission" SMALLINT DEFAULT 0
);

CREATE TABLE IF NOT EXISTS "role_permissions" ( -- `user` -> CRUD rights for `role`
    "user_id" BIGSERIAL REFERENCES "user" ON DELETE CASCADE,
    "resource_id" BIGINT REFERENCES "role" ON DELETE CASCADE DEFAULT NULL,
    "permission" SMALLINT DEFAULT 0
);

CREATE TABLE IF NOT EXISTS "group_permissions" ( -- `user` -> CRUD rights for `group`
    "user_id" BIGSERIAL REFERENCES "user" ON DELETE CASCADE,
    "resource_id" BIGINT REFERENCES "group" ON DELETE CASCADE DEFAULT NULL,
    "permission" SMALLINT DEFAULT 0
);

CREATE TABLE IF NOT EXISTS "file" (
    "uid" UUID PRIMARY KEY,
    "filename" VARCHAR(255),
    "type" VARCHAR(255),
    "location" VARCHAR(512),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "course" (
    "uid" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS "template" (
    "uid" BIGSERIAL PRIMARY KEY,
    "name" VARCHAR(255)
);

--- 1 course -> n content sections
CREATE TABLE IF NOT EXISTS "content_section" (
    "uid" BIGSERIAL PRIMARY KEY,
    "course_id" BIGINT NULL REFERENCES "course" ON DELETE CASCADE DEFAULT NULL,
    "template_id" BIGINT NULL REFERENCES "template" ON DELETE CASCADE DEFAULT NULL,
    "headline" VARCHAR(255),
    "order_index" INTEGER DEFAULT 0,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

--- Content Element = Daughter Element of Content Section
CREATE TABLE IF NOT EXISTS "content_element" (
    "uid" BIGSERIAL PRIMARY KEY,
    "section_id" BIGSERIAL REFERENCES "content_section" ON DELETE CASCADE,
    "order_index" INTEGER DEFAULT 0,
    "type" VARCHAR(255),
    "content" TEXT,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

--- 1:n multiple files, one content element
CREATE TABLE IF NOT EXISTS "file_in_content_element" (
    "content_id" BIGSERIAL REFERENCES "content_element" ON DELETE CASCADE,
    "file_id" UUID REFERENCES "file" ON DELETE CASCADE,
    "order_index" INTEGER DEFAULT 0,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

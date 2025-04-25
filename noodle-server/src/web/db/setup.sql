DROP TABLE IF EXISTS "user";

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

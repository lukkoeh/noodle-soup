FROM "postgres:17.5-alpine3.22"

ADD noodle-server/src/web/db/setup.sql /docker-entrypoint-initdb.d/01-setup.sql
ADD noodle-server/src/web/db/default_data.sql /docker-entrypoint-initdb.d/02-default-data.sql

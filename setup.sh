#! /bin/bash

set -a; source .env; set +a

createdb -h $PG_HOST -p $PG_PORT -U $PG_USER -e -E UTF-8 $PG_DB

psql -h $PG_HOST -p $PG_PORT -U $PG_USER $PG_DB < src/web/db/setup.sql

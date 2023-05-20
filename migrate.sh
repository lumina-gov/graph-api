#!/bin/bash

PSQLDEF_URL=https://github.com/k0kubun/sqldef/releases/download/v0.15.26/psqldef_linux_amd64.tar.gz

if [ ! -f psqldef ]
then
    echo "Downloading psqldef"
    curl -s -L $PSQLDEF_URL | tar xz
    echo "psqldef found"
else
    echo "psqldef found"
fi

# load env files
set -o allexport; source .env; set +o allexport

# make sure they're all set
for var_name in PG_HOST PG_USER PG_DATABASE PG_PASSWORD; do
    if [ -z "${!var_name}" ]; then 
        echo "ERROR: $var_name is not defined"; exit 1
    fi
done

# generate schema.sql: ./psqldef -U $PG_USER -W $PG_PASSWORD -h $PG_HOST $PG_DATABASE --export > schema.sql

# run migration
./psqldef -U $PG_USER -W $PG_PASSWORD -h $PG_HOST $PG_DATABASE --skip-drop < schema.sql
#!/bin/bash

# install atlas via >>
# curl -sSf https://atlasgo.sh | s

set -o allexport; source .env; set +o allexport

atlas schema apply -u $DATABASE_URL --to file://schema.sql --dev-url "docker://postgres/15/test";



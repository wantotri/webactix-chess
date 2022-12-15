#!/bin/bash

docker run -d --rm \
    --name chess-mongo \
    -p 27017:27017 \
    -v chess-data:/data/db \
    -e MONGO_INITDB_ROOT_USERNAME=admin \
    -e MONGO_INITDB_ROOT_PASSWORD=admin1234 \
    mongo:latest

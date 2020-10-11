#!/bin/sh

branch=$(git rev-parse --abbrev-ref HEAD)

if [ $branch != "master" ]
then 
    echo "Needs to be on master to bump current version"
    exit 1
fi

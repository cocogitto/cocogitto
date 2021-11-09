#!/bin/sh

branch=$(git rev-parse --abbrev-ref HEAD)

if [ $branch != "main" ]
then 
    echo "Needs to be on main to bump current version"
    exit 1
fi

#!/bin/bash -e

# Script used to detect changes in the Montrose MCP API

cd "$(dirname "$0")"

cargo r -F __dev --example=devel-introspect > new_api.json

if diff --color -u api.json new_api.json; then
    echo "API is unchanged"
    exit 0
else
    echo "API has changed"
    exit 1
fi

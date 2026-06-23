#!/bin/bash -e

# Script used to detect changes in the Montrose MCP API

cd "$(dirname "$0")"

cargo -q r -F __dev --example=devel-introspect > new_api.json

set +e
DIFF=$(diff -u api.json new_api.json)
if [[ $? -eq 0 ]]; then
    echo "API is unchanged"
    exit 0
else
    echo "API has changed" >&2
    echo "$DIFF"
    exit 1
fi

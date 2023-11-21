#!/usr/bin/env bash

set -ex

readonly DESTINATION="$1"

if [ -z "$DESTINATION" ]; then
    exit 1
fi

function rsync_to_remote() {
    local source="$1"
    local destination="$2"
    local host="$3"
    
    if [ -z "$source" ] || [ -z "$destination" ] || [ -z "$host" ]; then
        return 1
    fi
    
    rsync -avz --progress --rsync-path="sudo rsync" "$source" "$host:$destination"
}

rsync_to_remote target/release/bloop-core /usr/local/bin/bloop-core "${DESTINATION}"
rsync_to_remote scripts/bloop.service /etc/systemd/system "${DESTINATION}"
rsync_to_remote scripts/jackd.service /etc/systemd/system "${DESTINATION}"

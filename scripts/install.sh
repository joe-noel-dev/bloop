#!/usr/bin/env bash

set -e
set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly DESTINATION=joe@bloop.local

function rsync_to_remote() {
    local source="$1"
    local destination="$2"
    local host="$3"
    
    if [ -z "$source" ] || [ -z "$destination" ] || [ -z "$host" ]; then
        echo "Usage: rsync_to_remote <source> <destination> <host>"
        return 1
    fi
    
    rsync -avz --progress --rsync-path="sudo rsync" "$source" "$host:$destination"
}

rsync_to_remote target/release/bloop-core /usr/local/bin/bloop-core "${DESTINATION}"
rsync_to_remote scripts/bloop.service /etc/systemd/system "${DESTINATION}"
rsync_to_remote scripts/jackd.service /etc/systemd/system "${DESTINATION}"

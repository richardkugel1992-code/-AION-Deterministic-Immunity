#!/bin/sh
while true; do
    if ! pgrep -f "aion_scanner" > /dev/null; then
        nohup /root/aion_scanner/target/debug/aion_scanner > /dev/null 2>&1 &
    fi
    sleep 60
done

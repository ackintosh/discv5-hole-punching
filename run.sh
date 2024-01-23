#!/bin/bash

set -ex

if [ "$#" -eq 0 ]; then
  echo "Usage: $0 actor [internet_subnet] [router_ip]"
  exit 1
fi

if [ "$#" -eq 3 ]; then
  internet_subnet=$2
  router_ip=$3

  ip route add "$internet_subnet" via "$router_ip" dev eth0
fi


# debug
echo "" > /tmp/udp-hole-punching.log
case $1 in
  "initiator")
    sleep 3
    python /scripts/client.py 10.0.0.30 9000 &
    ;;
  "relay")
    python /scripts/server.py 0.0.0.0 9000 &
    ;;
  "target")
    sleep 3
    python /scripts/client.py 10.0.0.30 9000 &
    ;;
  *)
    echo "Invalid actor name"
    exit 1
    ;;
esac

#echo "1" > /tmp/setup_done # This will be checked by our docker HEALTHCHECK
#tail -f /dev/null # Keep it running forever.

# debug
tail -f /tmp/udp-hole-punching.log

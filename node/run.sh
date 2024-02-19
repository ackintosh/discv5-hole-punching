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

until redis-cli -h redis ping > /dev/null; do
  echo "Redis is not available - sleeping"
  sleep 1
done

RUST_LOG=discv5=trace /usr/local/bin/discv5-hole-punching "$1"

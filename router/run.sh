#!/bin/bash

set -ex

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 internal_host_ip"
  exit 1
fi

# `initiator` or `target` host ip address
INTERNAL_HOST_IP=$1

EXTERNAL_IF_ADDR=$(ip -4 -json addr | jq '.[] | select(.addr_info[]?.broadcast == "10.0.0.255").addr_info[0].local' -r)
EXTERNAL_IF_NAME=$(ip -4 -json addr | jq '.[] | select(.addr_info[]?.broadcast == "10.0.0.255").ifname' -r)

# ###############################################
# Set up nftables to simulate Restricted Cone NAT
# ###############################################
nft add table ip nat
# SNAT
nft add chain ip nat postrouting { type nat hook postrouting priority 100 \; }
nft add rule nat postrouting oifname "$EXTERNAL_IF_NAME" snat to "$EXTERNAL_IF_ADDR"
# DNAT
nft add chain ip nat prerouting { type nat hook prerouting priority -100 \; }
nft add rule nat prerouting iifname "$EXTERNAL_IF_NAME" dnat to "$INTERNAL_HOST_IP"
# FORWARD (established,related)
nft add table ip filter
nft add chain ip filter forward { type filter hook forward priority 0 \; }
nft add rule ip filter forward iif "$EXTERNAL_IF_NAME" udp dport 1024-65535 ct state established,related counter accept
# FORWARD (new)
nft add rule ip filter forward iif "$EXTERNAL_IF_NAME" udp dport 1024-65535 ct state new counter drop

tail -f /dev/null # Keep it running forever.
# discv5-hole-punching

This repository contains:

- A Docker-based environment to simulate a network topology consisting of nodes behind a Restricted Cone NAT.
- Tests for Discv5 Hole Punching.

## How to run

Simply run `docker compose up` and the nodes will be up and running the test.

```bash
# First we can run `docker compose down` to clean up Redis data.
docker compose down \
&& docker compose up --build --exit-code-from initiator --abort-on-container-exit 
```

You should see the message below in your terminal when the test has been successfully completed. If there is anything wrong during the test run, some panic messages will appear in the terminal.

```
...
initiator-1         | Hole punching has been done successfully.
...
initiator-1         | Test completed successfully.
...
target-1            | Test completed successfully.
...
relay-1             | Test completed successfully.
```

## Network Topology

The network consist of three nodes (initiator / target / relay) and two routers.

- **initiator**: a node that is behind a NAT, trying to establish a session with the target node
- **target**: a node that is behind a NAT
- **relay**: a node that is able to communicate with both initiator and target
- **initiator/target router**: a router that has two interfaces: one connected to an external network where the relay node exists, and another one connected to an internal network where the initiator/target node exists. This router forwards packets between the external and internal networks, behaving as a Restricted Cone NAT. The behavior is simulated using nftables. See router/run.sh.

Also, three segments in the network.

- **10.0.0.0/24**: relay node exists in this segment. We assume this segment represents the internet in this simulation.
- **192.168.0.0/24** and **172.16.0.0/24**: initiator and target node exist in these LAN segments.

```mermaid
graph TD
  relay_node[<center>10.0.0.30/24</center>]
  relay_node---target_router[<center>10.0.0.40/24<br><br>172.16.0.40/24</center>]
  relay_node---initiator_router[<center>10.0.0.20/24<br><br>192.168.0.20/24</center>]
  
  initiator_router---initiator_node(<center>192.168.0.10/24</center>)
  target_router---target_node(<center>172.16.0.50/24</center>)

  subgraph relay
    relay_node
  end

  subgraph initiator router
   initiator_router
  end
  
  subgraph target router
   target_router
  end
  
  subgraph initiator
   initiator_node
  end
  
  subgraph target
   target_node
  end
```

## Test flow

```mermaid
sequenceDiagram
    participant initiator
    participant relay
    participant target

    Note over initiator,target: Test step: preparing

    rect rgb(100, 100, 100)
    Note left of target: target sends PING <br> to establish session with relay.
    target->>relay: PING
    relay->>target: PONG
    Note over relay,target: Session established
    end

    Note over initiator,target: Test step: execution

    initiator->>relay: FINDNODE
    relay->>initiator: NODES
    Note over initiator: initiator sends a FINDNODE query to target, <br> but it will time out because target is behind NAT and <br> has not been hole-punched.
    initiator-->>target: FINDNODE

    Note over initiator: Due to the timeout, initiator sends RELAYINIT to relay to initiate hole-punching.
    initiator->>relay: RELAYINIT

    relay->>target: RELAYMSG
    target->>initiator: WHOAREYOU
    initiator->>target: FINDNODE
    target->>initiator: NODES

    Note over initiator,target: Test step: checking <br> the NODES response and their DHT entries.
```

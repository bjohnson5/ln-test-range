# Simple Interoperability Simulation
Test Network:
- 12 Nodes (4 LND, 4 LDK, 4 CLN)
- 6 Channels
- 160 Transactions
- 60sec Simulation

## Layout

### Group 1 (10 nodes, 6 channels, 160 transactions)
```
lnd-0000 --> cln-0000
cln-0001 --> lnd-0001
lnd-0002 --> ldk-0000 --> cln-0002
cln-0003 --> ldk-0001 --> lnd-0003
```

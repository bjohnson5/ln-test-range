# Big Interoperability Simulation
Test Network:
- 100 Nodes (45 LND, 45 LDK, 10 CLN)
- 133 Channels
- 282 Transactions
- 60sec Simulation

LND Node (lnd-0004):
- 40 Outgoing Channels
- 3 Incoming Channels
- 43 Peers

## Layout

### Group 1 (10 nodes, 6 channels, 240 transactions)
```
lnd-0000 (2000 sats every 1s) --> cln-0000
cln-0001 (3000 sats every 1s) --> lnd-0001
lnd-0002 (4000 sats every 1s) --> ldk-0000 --> cln-0002
cln-0003 (5000 sats every 1s) --> ldk-0001 --> lnd-0003
```

### Group 2 (83 nodes, 120 channels, 24 transactions)
```
lnd-0004 (6000 sats every 5s)
       |--> lnd-0005 --> ldk-0005 -->
       |--> lnd-0006 --> ldk-0006 -->
       |--> lnd-0007 --> ldk-0007 -->
       |--> lnd-0008 --> ldk-0008 -->
       |--> lnd-0009 --> ldk-0009 -->
       |--> lnd-0010 --> ldk-0010 -->
       |--> lnd-0011 --> ldk-0011 -->
       |--> lnd-0012 --> ldk-0012 -->
       |--> lnd-0013 --> ldk-0013 -->
       |--> lnd-0014 --> ldk-0014 -->
       |--> lnd-0015 --> ldk-0015 -->
       |--> lnd-0016 --> ldk-0016 -->
       |--> lnd-0017 --> ldk-0017 -->
       |--> lnd-0018 --> ldk-0018 -->
       |--> lnd-0019 --> ldk-0019 -->
       |--> lnd-0020 --> ldk-0020 -->
       |--> lnd-0021 --> ldk-0021 -->
       |--> lnd-0022 --> ldk-0022 -->
       |--> lnd-0023 --> ldk-0023 -->
       |--> lnd-0024 --> ldk-0024 -->
                                    | --> cln-0005
(7000 sats every 5s)
       |--> ldk-0025 --> lnd-0025 -->
       |--> ldk-0026 --> lnd-0026 -->
       |--> ldk-0027 --> lnd-0027 -->
       |--> ldk-0028 --> lnd-0028 -->
       |--> ldk-0029 --> lnd-0029 -->
       |--> ldk-0030 --> lnd-0030 -->
       |--> ldk-0031 --> lnd-0031 -->
       |--> ldk-0032 --> lnd-0032 -->
       |--> ldk-0033 --> lnd-0033 -->
       |--> ldk-0034 --> lnd-0034 -->
       |--> ldk-0035 --> lnd-0035 -->
       |--> ldk-0036 --> lnd-0036 -->
       |--> ldk-0037 --> lnd-0037 -->
       |--> ldk-0038 --> lnd-0038 -->
       |--> ldk-0039 --> lnd-0039 -->
       |--> ldk-0040 --> lnd-0040 -->
       |--> ldk-0041 --> lnd-0041 -->
       |--> ldk-0042 --> lnd-0042 -->
       |--> ldk-0043 --> lnd-0043 -->
       |--> ldk-0044 --> lnd-0044 -->
                                    | --> cln-0006
```

### Group 3 (7 nodes, 7 channels, 18 transactions)
```
cln-0007 (8000 sats every 10s) --> ldk-0002 --> ldk-0005 --> lnd-0004
cln-0008 (9000 sats every 10s) --> ldk-0003 ---------------> |
cln-0009 (9500 sats every 10s) --> ldk-0004 ---------------> |
```

# Network topography simulator
### Simulate packet hopping in a network topology graph 

See [this exercise](https://github.com/JayTee42/tubaf-rn-2020-21/blob/main/02%20-%20ISO_OSI/HA.pdf) from my computer network course for details.

```
cargo run --release -- line 1000
```

There are four topologies:
 - Ring (`ring`)
 - Directed ring (`oneway_ring`)
 - Star (`star`)
 - Line (`line`)

The following number (which must be > 1) specifies the number of nodes in the graph.

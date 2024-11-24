# Home Network Daemon

The purpose of `hnd` is to provide a self-managing, simple daemon for
integrating into a home network. A common issue in home networks as they grow is
being able to manage names across devices. Most budget home routers will supply
a dhcp server to issue IP addresses and route traffic, but won't do much for
names. Or if they do, it's often a clunky web interface.

`hnd` replaces dhcp client and local resolver and in addition supplies a small
dhcp and dns server on each system where it runs. It runs a small discovery and
peer-to-peer communication channel to communicate with other `hnd` processes in
the network. These work together to learn the network and keep each other
informed so that local queries can use hostname lookups instead of making the
user memorize IP addresses or, worse, have to see which address has been
assigned this week.

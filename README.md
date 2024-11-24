# Home Network Daemon

The purpose of `hnd` is to provide a self-managing, simple daemon for
integrating into a home network. A common issue in home networks as they grow is
being able to manage names across devices. Most budget home routers will supply
a DHCP server to issue IP addresses and route traffic, but won't do much for
names. Or if they do, it's often a clunky web interface.

`hnd` replaces DHCP client and local resolver and in addition supplies a small
DHCP and recursing DNS server on each system where it runs. It runs a small
discovery and peer-to-peer communication channel to communicate with other `hnd`
processes in the network. These work together to learn the network and keep each
other informed so that local queries can use hostname lookups instead of making
the user memorize IP addresses or, worse, have to see which address has been
assigned this week.

This is built with standard protocls, so any other systems in the network can
still interoperate without issue.

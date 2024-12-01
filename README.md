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

## Sources

### ARP, BOOTP and DHCP

- [RFC1044](https://www.rfc-editor.org/rfc/rfc1044) _Internet Protocol on Network Systems HYPERchannel Protocol Specification_, February 1988
- [RFC1329](https://www.rfc-editor.org/rfc/rfc1329) _Thoughts on Address Resolution for Dual MAC FDDI Networks_, May 1992
- [RFC1542](http://www.rfc-editor.org/rfc/rfc1542) _Clarifications and Extensions for the Bootstrap Protocol_, October 1993
- [RFC2131](https://www.rfc-editor.org/rfc/rfc2131) _Dynamic Host Configuration Protocol_, March 1997
- [RFC2131](https://www.rfc-editor.org/rfc/rfc2131) _Dynamic Host Configuration Protocol_, March 1997
- [RFC2132](http://www.rfc-editor.org/rfc/rfc2132) _DHCP Options and BOOTP Vendor Extensions_, March 1997
- [RFC2132](https://www.rfc-editor.org/rfc/rfc2132) _DHCP Options and BOOTP Vendor Extensions_, March 1997
- [RFC2176](https://www.rfc-editor.org/rfc/rfc2176) _IPv4 over MAPOS Version 1_, June 1997
- [RFC2225](https://www.rfc-editor.org/rfc/rfc2225) _Classical IP and ARP over ATM_, April 1998
- [RFC2834](https://www.rfc-editor.org/rfc/rfc2834) _ARP and IP Broadcast over HIPPI-800_, May 2000
- [RFC2835](https://www.rfc-editor.org/rfc/rfc2835) _IP and ARP over HIPPI-6400 (GSN)_, May 2000
- [RFC3315](https://www.rfc-editor.org/rfc/rfc3315) _Dynamic Host Configuration Protocol for IPv6 (DHCPv6)_, July 2003
- [RFC3396](https://www.rfc-editor.org/rfc/rfc3396) _Encoding Long Options in the Dynamic Host Configuration Protocol (DHCPv4)_, November 2002
- [RFC3442](https://www.rfc-editor.org/rfc/rfc3442) _The Classless Static Route Option for Dynamic Host Configuration Protocol (DHCP) version 4_, December 2002
- [RFC3942](https://www.rfc-editor.org/rfc/rfc3942) _Reclassifying Dynamic Host Configuration Protocol version 4 (DHCPv4) Options_, November 2004
- [RFC4338](https://www.rfc-editor.org/rfc/rfc4338) _Transmission of IPv6, IPv4, and Address Resolution Protocol (ARP) Packets over Fibre Channel_, January 2006
- [RFC4361](https://www.rfc-editor.org/rfc/rfc4361) _Node-specific Client Identifiers for Dynamic Host Configuration Protocol Version Four (DHCPv4)_, February 2006
- [RFC4361](https://www.rfc-editor.org/rfc/rfc4361) _Node-specific Client Identifiers for Dynamic Host Configuration Protocol Version Four (DHCPv4)_, February 2006
- [RFC4701](https://www.rfc-editor.org/rfc/rfc4701) _A DNS Resource Record (RR) for Encoding Dynamic Host Configuration Protocol (DHCP) Information (DHCID RR)_, October 2006
- [RFC4833](https://www.rfc-editor.org/rfc/rfc4833) _Timezone Options for DHCP_, April 2007
- [RFC5494](https://www.rfc-editor.org/rfc/rfc5494) _IANA Allocation Guidelines for the Address Resolution Protocol (ARP)_, April 2009
- [RFC826](https://www.rfc-editor.org/rfc/rfc826) _An Ethernet Address Resolution Protocol or Converting Network Protocol Addresses to 48.bit Ethernet Address for Transmission on Ethernet Hardware_, November 1982
- [RFC951](http://www.rfc-editor.org/rfc/rfc951) _Bootstrap Protocol (BOOTP)_, September 1985
- [RFC951](https://www.rfc-editor.org/rfc/rfc951) _Bootstrap Protocol (BOOTP)_, September 1985
- Obsolete: [RFC1048](https://www.rfc-editor.org/rfc/rfc1048) _BOOTP Vendor Information Extensions_, February 1988
- Obsolete: [RFC1084](https://www.rfc-editor.org/rfc/rfc1084) _BOOTP Vendor Information Extensions_, December 1988
- Obsolete: [RFC1531](https://www.rfc-editor.org/rfc/rfc1531) _Dynamic Host Configuration Protocol_, October 1993
- Obsolete: [RFC1533](https://www.rfc-editor.org/rfc/rfc1533) _DHCP Options and BOOTP Vendor Extensions_, October 1993
- Obsolete: [RFC1541](https://www.rfc-editor.org/rfc/rfc1541) _Dynamic Host Configuration Protocol_, October 1993

### DNS

- [RFC1035](https://www.rfc-editor.org/rfc/rfc1035) _Domanin Names - Implementation and Specification_, November 1987
- [RFC1101](https://www.rfc-editor.org/rfc/rfc1101) _DNS Encoding of Network Names and Other Types_, April 1989
- [RFC1183](https://www.rfc-editor.org/rfc/rfc1183) _New DNS RR Definitions_, October 1990
- [RFC1348](https://www.rfc-editor.org/rfc/rfc1348) _DNS NSAP RRs_, July 1992
- [RFC1876](https://www.rfc-editor.org/rfc/rfc1876) _A Means for Expressing Location Information in the Domain Name System_, January 1996
- [RFC1982](https://www.rfc-editor.org/rfc/rfc1982) _Serial Number Arithmetic_, August 1996
- [RFC1995](https://www.rfc-editor.org/rfc/rfc1995) _Incremental Zone Transfer in DNS_, August 1996
- [RFC1996](https://www.rfc-editor.org/rfc/rfc1996) _A Mechanism for Prompt Notification of Zone Changes (DNS NOTIFY)_, August 1996
- [RFC2136](https://www.rfc-editor.org/rfc/rfc2136) _Dynamic Updates in the Domain Name System (DNS UPDATE)_, April 1997
- [RFC2137](https://www.rfc-editor.org/rfc/rfc2137) _Secure Domain Name System Dynamic Update_, April 1997
- [RFC2181](https://www.rfc-editor.org/rfc/rfc2181) _Clarifications to the DNS Specification_, July 1997
- [RFC2308](https://www.rfc-editor.org/rfc/rfc2308) _Negative Caching of DNS Queries (DNS NCACHE)_, March 1998
- [RFC2673](https://www.rfc-editor.org/rfc/rfc2673) _Binary Labels in the Domain Name System_, August 1999
- [RFC2845](https://www.rfc-editor.org/rfc/rfc2845) _Secret Key Transaction Authentication for DNS (TSIG)_, May 2000
- [RFC3425](https://www.rfc-editor.org/rfc/rfc3425) _Obsoleting IQUERY_, November 2002
- [RFC4033](https://www.rfc-editor.org/rfc/rfc4033) _DNS Security Introduction and Requirements_, March 2005
- [RFC4034](https://www.rfc-editor.org/rfc/rfc4034) _Resource Records for the DNS Security Extensions_, March 2005
- [RFC4035](https://www.rfc-editor.org/rfc/rfc4035) _Protocol Modifications for the DNS Security Extensions_, March 2005
- [RFC4343](https://www.rfc-editor.org/rfc/rfc4343) _Domain Name System (DNS) Case Insensitivity Clarification_, January 2006
- [RFC5936](https://www.rfc-editor.org/rfc/rfc5936) _DNS Zone Transfer Protocol (AXFR)_, June 2010
- [RFC5966](https://www.rfc-editor.org/rfc/rfc5966) _DNS Transport over TCP - Implementation Requirements_, August 2010
- [RFC6604](https://www.rfc-editor.org/rfc/rfc6604) _xNAME RCODE and Status Bits Clarification_, April 2012
- [RFC7766](https://www.rfc-editor.org/rfc/rfc7766) _DNS Transport over TCP - Implementation Requirements_, March 2016
- [RFC8482](https://www.rfc-editor.org/rfc/rfc8482) _Providing Minimal-Sized Responses to DNS Queries That Have QTYPE=ANY_, January 2019
- [RFC8490](https://www.rfc-editor.org/rfc/rfc8490) _DNS Stateful Operations_, March 2019
- [RFC8767](https://www.rfc-editor.org/rfc/rfc8767) _Serving Stale Data to Improve DNS Resiliency_, March 2020
- [RFC9619](https://www.rfc-editor.org/rfc/rfc9619) _In the DNS, QDCOUNT Is (Usually) One)_, July 2024
- Obsolete: [RFC2065](https://www.rfc-editor.org/rfc/rfc2065) _Domain Name System Security Extensions_, January 1997
- Obsolete: [RFC2535](https://www.rfc-editor.org/rfc/rfc2535) _Domain Name System Security Extensions_, March 1999
- Obsolete: [RFC3008](https://www.rfc-editor.org/rfc/rfc3008) _Domain Name System Security (DNSSEC) Signing Authority_, November 2000
- Obsolete: [RFC3090](https://www.rfc-editor.org/rfc/rfc3090) _DNS Security Extension Clarification on Zone Status_, March 2001
- Obsolete: [RFC3445](https://www.rfc-editor.org/rfc/rfc3445) _Limiting the Scope of the KEY Resource Record (RR)_, December 2002
- Obsolete: [RFC3655](https://www.rfc-editor.org/rfc/rfc3655) _Redefinition of DNS Authenticated Data (AD) bit_, November 2003
- Obsolete: [RFC3658](https://www.rfc-editor.org/rfc/rfc3658) _Delegation Signer (DS) Resource Record (RR)_, December 2003
- Obsolete: [RFC3755](https://www.rfc-editor.org/rfc/rfc3755) _Legacy Resolver Compatibility for Delegation Signer (DS)_, May 2004
- Obsolete: [RFC3757](https://www.rfc-editor.org/rfc/rfc3757) _Domain Name System KEY (DNSKEY) Resource Record (RR) Secure Entry Point (SEP) Flag_, April 2004
- Obsolete: [RFC3845](https://www.rfc-editor.org/rfc/rfc3845) _DNS Security (DNSSEC) NextSECure (NSEC) RDATA Format_, August 2004
- Obsolete: [RFC882](https://www.rfc-editor.org/rfc/rfc882) _Domain Names - Concepts and Facilities_, November 1983
- Obsolete: [RFC883](https://www.rfc-editor.org/rfc/rfc883) _Domain Names - Implementation and Specification_, November 1983
- Obsolete: [RFP973](https://www.rfc-editor.org/rfc/rfc973) _Domain System Changes and Observations_, January 1986
- [RFC1034](https://www.rfc-editor.org/rfc/rfc1034) _Domain Names - Concepts and Faciliites_, November 1987

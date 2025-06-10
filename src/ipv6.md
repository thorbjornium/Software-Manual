# IPv6

<!-- toc -->

## Unicast

- **One-to-one communication**: Data is sent from a single source to a single, specific destination address.
- **Typical use**: Most internet traffic (web browsing, file downloads) is unicast.
- **IPv6 example**: Sending a packet from your computer to a specific server.

### 1. Global Unicast Address

- **Publicly routable on the Internet**
- Prefix: `2000::/3`
- Used for global communication

### 2. Unique Local Address (ULA)

- **Private, not routable on the Internet**
- Prefix: `FD00::/8`
- Used for local communications within a site

### 3. Link-Local Address

- **Valid only within a single local network segment**
- Prefix: `FE80::/10`
- Automatically assigned to every IPv6 interface

### 4. Loopback Address

- **Used by a device to send packets to itself**
- Address: `::1/128`

### 5. Unspecified Address

- **Indicates the absence of an address**
- Address: `::/128`

### 6. IPv4-Embedded IPv6 Addresses

- **Allow IPv4 and IPv6 interoperability**
- Embed a 32-bit IPv4 address within an IPv6 address, often for transition mechanisms like NAT64, IPv4-mapped, and compatible addresses.
- Common formats:
  - **IPv4-mapped IPv6**: `::ffff:192.0.2.33` (80 bits zero, 16 bits ones, 32 bits IPv4)  
    Used for representing IPv4 addresses in IPv6-only environments, especially in software and NAT64 scenarios
  - **IPv4-compatible IPv6**: `::192.0.2.33` [deprecated; 96 bits zero, 32 bits IPv4]
  - **NAT64/XLAT464**: Uses a configurable prefix (e.g., `64:ff9b::/96`), followed by the IPv4 address
  - **Custom Embedded**: Networks can embed IPv4 addresses using various prefix lengths, placing the IPv4 address at specific bits within the IPv6 address

#### Example Table

| Type/Format                | Example Address                 | Description                                              |
|----------------------------|---------------------------------|----------------------------------------------------------|
| Global Unicast             | 2001:db8::1                     | Public, Internet-routable                                |
| Unique Local (ULA)         | fd12:3456:789a::1               | Private, site-local only                                 |
| Link-Local                 | fe80::1                         | Local segment only                                       |
| Loopback                   | ::1                             | Local device (self)                                      |
| Unspecified                | ::                              | No specific address                                      |
| IPv4-mapped IPv6           | ::ffff:192.0.2.33               | IPv4 in IPv6 for dual-stack/NAT64                        |
| IPv4-compatible IPv6       | ::192.0.2.33                    | Deprecated, for dual-stack                               |
| NAT64/XLAT464 Embedded     | 64:ff9b::192.0.2.33             | NAT64 translation prefix + IPv4                          |
| Custom Embedded (various)  | 2001:db8::c000:221              | Prefix + embedded IPv4 (e.g., 192.0.2.33)                |

**Note:** Embedding IPv4 in IPv6 addresses is essential for transition and interoperability between IPv4 and IPv6 networks

## Multicast

- **One-to-many communication**: Data is sent from one source to multiple destinations, but only to devices that have joined a specific multicast group.
- **Efficient for group communication**: Saves bandwidth because the data is only sent once and delivered to all interested receivers.
- **Typical use**: Streaming media, online conferencing, or network discovery protocols.
- **IPv6 example**: Sending a video stream to all devices in a multicast group (e.g., all routers on a network).

### General Structure

All IPv6 multicast addresses start with the prefix `FF00::/8` (the first 8 bits are `11111111` in binary, or `FF` in hexadecimal).

#### Multicast Address Format

| Bits | Field   | Description                                                                                   |
|------|---------|-----------------------------------------------------------------------------------------------|
| 8    | Prefix  | Always `11111111` (FF) to indicate multicast                                                 |
| 4    | Flags   | Various control flags (see below)                                                            |
| 4    | Scope   | Defines the reach of the multicast (e.g., link-local, global)                                |
| 112  | Group ID| Identifies the multicast group                                                               |

Example: `FF02::1` (All nodes on the local link)

---

### Flags Field

The 4-bit flags field provides additional information about the multicast address:

| Bit | Flag | Meaning when 0                  | Meaning when 1                       |
|-----|------|---------------------------------|--------------------------------------|
| 8   | Reserved | Reserved                    | Reserved                             |
| 9   | R (Rendezvous) | RP not embedded       | RP embedded                          |
| 10  | P (Prefix) | Not based on prefix       | Based on network prefix              |
| 11  | T (Transient) | Well-known address     | Dynamically assigned address         |

---

### Scope Field

The 4-bit scope field defines where the multicast is valid:

| Scope Value | IPv6 Address Example | Scope Name         | Description                                                      |
|-------------|---------------------|--------------------|------------------------------------------------------------------|
| 1           | FF01::/16           | Interface-local    | Stays within the originating node                                |
| 2           | FF02::/16           | Link-local         | Stays within the local link                                      |
| 3           | FF03::/16           | Realm-local        | Local to a particular network technology                         |
| 4           | FF04::/16           | Admin-local        | Administratively configured scope                                |
| 5           | FF05::/16           | Site-local         | Restricted to the local site                                     |
| 8           | FF08::/16           | Organization-local | Restricted to the organization                                   |
| E           | FFE0::/16           | Global             | Globally routable multicast                                      |

---

### Special Multicast Address Types

- **Solicited-Node Multicast Address:** Used by Neighbor Discovery Protocol (NDP). Format: `FF02::1:FFXX:XXXX` where the last 24 bits come from the unicast/anycast address.
- **Well-Known Multicast Groups:** Assigned by IANA for specific protocols (e.g., `FF02::1` for all nodes, `FF02::2` for all routers).

---

### Examples

| Address       | Scope      | Group Purpose                |
|---------------|------------|------------------------------|
| FF02::1       | Link-local | All nodes on the local link  |
| FF02::2       | Link-local | All routers on the local link|
| FF05::101     | Site-local | NTP servers in a site        |
| FF0E::1       | Global     | All nodes (global scope)     |

---

### Summary

- All IPv6 multicast addresses begin with `FF`.
- The next 4 bits are flags, followed by 4 bits for scope.
- The remaining 112 bits specify the multicast group.
- Scope and flags determine the reach and type of multicast group.
- No broadcast in IPv6; multicast fulfills this role.

---

## Anycast

- **One-to-nearest communication**: Data is sent from one source to the nearest (in routing terms) member of a group of receivers sharing the same address.
- **Used for redundancy and load balancing**: Improves availability and performance by routing requests to the closest server.
- **Typical use**: DNS root servers, content delivery networks (CDNs), and global services like Google DNS.
- **IPv6 example**: Multiple servers around the world share the same anycast address; a user’s request is automatically routed to the closest server.

### What is an Anycast Address?

An IPv6 anycast address is an address assigned to multiple interfaces, typically on different nodes. When a packet is sent to an anycast address, it is routed to the "nearest" interface (according to the routing protocol's metric, such as hop count or cost) that has been assigned that address.

---

### Key Characteristics

- **Indistinguishable from Unicast:** Anycast addresses are allocated from the unicast address space and look identical to unicast addresses. There is no special prefix or format to distinguish them; their "anycast" property comes from being assigned to multiple interfaces.
- **Routing Behavior:** Routers deliver packets sent to an anycast address to the closest (in terms of routing distance) node with that address.
- **Use Cases:** Anycast is commonly used for:
  - Redundancy and load balancing (e.g., DNS root servers, CDN edge nodes)
  - Service discovery and high availability
  - Routing to the nearest gateway or service provider

---

### IPv6 Anycast Address Examples

- **Subnet Router Anycast:** The lowest address in any IPv6 subnet (where the interface identifier is all zeros) is reserved as the subnet-router anycast address.

For example, in the subnet:
> 2001:db8:abcd:0012::/64

the anycast address is:
> 2001:db8:abcd:12::

- **Reserved Ranges:** The highest 128 interface identifiers in a subnet are also reserved for anycast purposes.

---

### How Anycast Works

1. **Assignment:** The same unicast address is configured on multiple devices/interfaces.
2. **Routing:** Each device advertises its route to the shared address. Routers learn multiple paths to the same address.
3. **Packet Delivery:** When a host sends a packet to the anycast address, the network delivers it to the nearest device (according to the routing protocol).

---

### Summary Table

| Feature         | Description                                                                 |
|-----------------|-----------------------------------------------------------------------------|
| Address Format  | Identical to unicast                                                        |
| Assignment      | Assigned to multiple interfaces/nodes                                       |
| Routing         | Delivered to the nearest interface with the address                         |
| Common Uses     | DNS root servers, CDN nodes, redundant gateways, service discovery          |
| IPv6 Reserved   | Subnet-router anycast (all-zero interface ID), highest 128 interface IDs    |

---

**Note:** Anycast addresses provide a way to distribute services across multiple locations, improving redundancy, availability, and response times for clients.

 <br>

| Type      | Communication Model | Use Case Example                | IPv6 Address Range/Assignment      |
|-----------|--------------------|----------------------------------|------------------------------------|
| Unicast   | One-to-one         | Web browsing, file download      | Global: 2000::/3, Link-local: FE80::/10, ULA: FC00::/7 |
| Multicast | One-to-many        | Video streaming, network discovery | FF00::/8                          |
| Anycast   | One-to-nearest     | DNS root servers, CDNs           | Allocated from unicast space       |

**Note:** IPv6 does not use broadcast; multicast replaces broadcast functionality.

## ND, NS, and ULA: The Essentials

**IPv6 ND (Neighbor Discovery), NS (Neighbor Solicitation), and ULA (Unique Local Address)** are foundational concepts in IPv6 networking. Here’s a concise breakdown:

---

**IPv6 Neighbor Discovery (ND)**:

- ND is a protocol in IPv6 that allows devices (nodes) on the same network link to discover each other, learn about routers, resolve Layer 3 (IP) to Layer 2 (MAC) addresses, and maintain reachability information.
- It replaces IPv4’s ARP and ICMP Router Discovery, providing more secure and efficient communication.
- ND uses several ICMPv6 message types, including Neighbor Solicitation (NS) and Neighbor Advertisement (NA), to perform its functions.

**Key ND Functions:**

- Address resolution (finding a device’s MAC address from its IPv6 address)
- Neighbor reachability detection
- Router and prefix discovery
- Address autoconfiguration (stateless)
- Redirecting hosts to better routes

---

**Neighbor Solicitation (NS)**:

- NS is a specific message type within ND, used to discover the link-layer (MAC) address of a neighbor or to verify its reachability.
- It is functionally similar to an ARP Request in IPv4 but uses multicast instead of broadcast, making it more efficient.
- NS messages are sent to a solicited-node multicast address derived from the target IPv6 address, ensuring only the relevant device responds.

**How it works:**

- A host sends an NS message to resolve a neighbor’s MAC address.
- The target device replies with a Neighbor Advertisement (NA), providing its MAC address.
- NS is also used for Duplicate Address Detection (DAD), ensuring no two devices use the same IPv6 address on the network.

---

**Unique Local Address (ULA)**:

- ULAs are IPv6 addresses meant for local communications within a site or organization, similar to private IPv4 addresses (like 192.168.x.x).
- They are not routable on the global Internet but can be routed within private networks.
- ULA addresses start with the prefix **FD00::/8**. The rest of the address is generated randomly to ensure uniqueness within the local scope.

**Key ULA Points:**

- Used for internal-only communication
- Not registered or routed on the Internet
- Replace the deprecated “site-local” IPv6 addresses

---

### Summary Table

| Term | Purpose | Key Features |
| :-- | :-- | :-- |
| ND (Neighbor Discovery) | Device discovery, address resolution, router/prefix discovery | Replaces ARP, uses ICMPv6, supports autoconfiguration and reachability detection |
| NS (Neighbor Solicitation) | Resolve MAC addresses, check reachability | Multicast-based, more efficient than ARP, used for DAD|
| ULA (Unique Local Address) | Private IPv6 addressing | FD00::/8 prefix, not Internet-routable, for internal use |

---

These mechanisms are essential for IPv6 networks to operate efficiently, securely, and with minimal manual configuration.

## Global Unicast Address, GUA

An IPv6 Global Unicast Address (GUA) is a globally unique, Internet-routable address, equivalent to a public IPv4 address. GUAs are used for devices that need to communicate across the Internet and are assigned by Internet authorities to ensure uniqueness.

---

### Structure of an IPv6 GUA

A typical IPv6 GUA consists of three main parts:

| Part                   | Length    | Description                                                                 |
|------------------------|-----------|-----------------------------------------------------------------------------|
| Global Routing Prefix  | 48 bits   | Uniquely identifies the network; assigned by IANA, RIR, or ISP              |
| Subnet ID              | 16 bits   | Identifies subnets within the organization; set by local administrators     |
| Interface ID           | 64 bits   | Uniquely identifies an interface on a subnet; usually auto-generated        |

- **Prefix:** All GUAs start with binary `001` (hex 2 or 3), corresponding to `2000::/3`.

**Example Address:**

>2001:0db8:1234:5678:9abc:def0:1234:5678

---

### Breakdown of Address Parts

#### 1. Global Routing Prefix (48 bits)

- Assigned hierarchically (IANA → RIR → ISP → organization)
- Ensures global uniqueness
- Used by routers to forward packets across the Internet

#### 2. Subnet ID (16 bits)

- Allows organizations to divide their address space into multiple subnets
- Functions similarly to subnetting in IPv4, but with more flexibility and scale
- Expressed in hexadecimal notation

#### 3. Interface ID (64 bits)

- Uniquely identifies each interface within a subnet
- Often auto-generated from the device’s MAC address using EUI-64, but can be set manually
- Ensures each device on a subnet has a unique address

---

### Allocation Example

- **IANA** assigns a large block (e.g., `2001::/16`) to a Regional Internet Registry (RIR)
- **RIR** assigns a smaller block (e.g., `2001:0db8::/32`) to an ISP
- **ISP** assigns a /48 prefix (e.g., `2001:0db8:abcd::/48`) to an organization
- **Organization** subdivides the /48 into multiple /64 subnets using the Subnet ID
- **Devices** on each subnet get a unique Interface ID

---

### Key Features

- **Globally unique and routable:** Ensures no address conflicts on the Internet
- **Hierarchical allocation:** Simplifies routing and address management
- **Supports subnetting:** Organizations can create many subnets internally
- **Large address space:** 64-bit Interface ID allows for vast numbers of devices per subnet

---

### Summary Table

| Field                  | Length (bits) | Example Value         | Purpose                                  |
|------------------------|---------------|----------------------|------------------------------------------|
| Global Routing Prefix  | 48            | 2001:0db8:abcd       | Identifies organization/network globally |
| Subnet ID              | 16            | 1234                 | Identifies subnet within organization    |
| Interface ID           | 64            | 5678:9abc:def0:1234  | Identifies device/interface uniquely     |

---

**In summary:**  
IPv6 GUAs are globally unique, Internet-routable addresses with a hierarchical structure, enabling scalable, efficient, and conflict-free global networking.

## EUI: Calculating IPv6 Interface ID Using EUI-64 from MAC Address 1C-6F-65-C2-BD-F8

### Step-by-Step EUI-64 Calculation

1. **Split the MAC address into two halves:**
   - MAC: `1C-6F-65-C2-BD-F8`
   - First half: `1C-6F-65`
   - Second half: `C2-BD-F8`

2. **Insert `FFFE` in the middle:**
   - Result: `1C-6F-65-FF-FE-C2-BD-F8`

3. **Convert to IPv6 notation (grouped by two bytes):**
   - `1C6F:65FF:FEC2:BDF8`

4. **Invert the 7th bit (Universal/Local bit) of the first byte:**
   - First byte `1C` in binary: `00011100`
   - 7th bit (counting from left, zero-based): currently `0`
   - Invert it to `1`: `00011110` (hex: `1E`)
   - So, the new first byte is `1E`

5. **Final EUI-64 Interface ID:**
   - Replace the first byte: `1E6F:65FF:FEC2:BDF8`

### **Result**

The interface ID, generated using EUI-64 from the MAC address `1C-6F-65-C2-BD-F8`, is: `1E6F:65FF:FEC2:BDF8`

## IPv4 Header Fields vs IPv6 Header Fields

IPv6 simplifies and streamlines the IP header compared to IPv4, removing or replacing several fields to improve processing efficiency and support new features.

### Comparison Table: IPv4 vs IPv6 Header Fields

| IPv4 Header Field      | IPv6 Header Field     | Present in IPv6? | Description / Notes                                                                 |
|----------------------- |---------------------- |------------------|-------------------------------------------------------------------------------------|
| Version                | Version               | Yes              | 4 bits; indicates IP version (4 or 6)                                               |
| IHL (Header Length)    | —                     | No               | IPv6 header is fixed at 40 bytes; field not needed                                  |
| Type of Service (ToS)  | Traffic Class         | Yes              | 8 bits; used for QoS (DSCP marking)                                                 |
| Total Length           | Payload Length        | Yes              | In IPv4, includes header + data; in IPv6, only payload + extension headers          |
| Identification         | —                     | No               | Used for fragmentation in IPv4; handled via extension header in IPv6                 |
| Flags                  | —                     | No               | Used for fragmentation in IPv4; handled via extension header in IPv6                 |
| Fragment Offset        | —                     | No               | Used for fragmentation in IPv4; handled via extension header in IPv6                 |
| Time to Live (TTL)     | Hop Limit             | Yes              | 8 bits; limits packet lifetime (prevents loops)                                      |
| Protocol               | Next Header           | Yes              | 8 bits; indicates next header or upper-layer protocol (TCP, UDP, etc.)              |
| Header Checksum        | —                     | No               | Removed in IPv6; error checking handled elsewhere                                    |
| Source Address         | Source Address        | Yes              | 32 bits in IPv4, 128 bits in IPv6                                                    |
| Destination Address    | Destination Address   | Yes              | 32 bits in IPv4, 128 bits in IPv6                                                    |
| Options                | Extension Headers     | Yes (different)  | IPv4 options replaced by separate extension headers in IPv6                          |
| Padding                | —                     | No               | Not present; IPv6 header is naturally aligned                                        |
| Flow Label             | Flow Label            | Yes (new)        | 20 bits; used for labeling packets belonging to the same flow (new in IPv6)          |

---

### Key Differences

- **IPv6 header is fixed at 40 bytes** (IPv4 header is variable: 20–60 bytes).
- **Fragmentation fields** (Identification, Flags, Fragment Offset) are removed from the main header and handled by an optional extension header in IPv6.
- **Header checksum** is eliminated in IPv6 to reduce processing overhead.
- **Options** are replaced by more flexible **extension headers** in IPv6.
- **Flow Label** is a new field in IPv6 for special packet handling.
- **Address fields** are expanded from 32 bits (IPv4) to 128 bits (IPv6).

---

### Visual Summary

| Field            | IPv4                   | IPv6                   |
|------------------|------------------------|------------------------|
| Version          | 4                      | 6                      |
| Header Length    | Variable (20–60 bytes) | Fixed (40 bytes)       |
| Source Address   | 32 bits                | 128 bits               |
| Destination Addr | 32 bits                | 128 bits               |
| Checksum         | Yes                    | No                     |
| Extension Header | No (Options instead)   | Yes                    |
| Flow Label       | No                     | Yes (20 bits)          |

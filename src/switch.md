# Switch

<!-- toc -->

## Summary Table of Switch Methods

| Method             | How It Works                                                                                   | Speed        | Error Checking    | Frame Processing Start      | Notes / Use Cases                                    |
|--------------------|-----------------------------------------------------------------------------------------------|--------------|-------------------|----------------------------|------------------------------------------------------|
| Store-and-Forward  | Receives entire frame, checks for errors (CRC), then forwards if valid.                       | Slowest      | Best (CRC check)  | After full frame received  | High accuracy, supports different port speeds         |
| Cut-Through        | Starts forwarding after reading destination MAC (first 6–8 bytes), before whole frame arrives. | Fastest      | None              | After 1st 6–8 bytes        | Lowest latency, may forward corrupted frames          |
| - Fragment-Free      | Waits for first 64 bytes (to filter out runts), then forwards.                                | Moderate     | Limited (runts)   | After 64 bytes received    | Balances speed and error avoidance (filters runts)    |
| - Fast-Forward       |**Often used as another term for cut-through** (vendor-specific); behaves like cut-through.        | Fastest      | None              | After 1st 6–8 bytes        | Sometimes used interchangeably with cut-through       |

---

### Method Details

#### **Store-and-Forward**

- Switch receives the entire Ethernet frame into memory.
- Performs a CRC (Cyclic Redundancy Check) to detect errors.
- Only forwards error-free frames.
- Introduces the most latency but ensures data integrity.
- Can connect ports of different speeds.

#### **Cut-Through**

- Switch reads only enough of the frame to learn the destination MAC (typically first 6–8 bytes).
- Starts forwarding immediately after determining the output port.
- Lowest latency, but may forward frames with errors.
- Cannot detect or filter out corrupted frames.

#### **- Fragment-Free**

- Switch waits for the first 64 bytes of the frame before forwarding.
- 64 bytes is the minimum legal Ethernet frame size; anything smaller is a "runt" (likely a collision fragment).
- Filters out runts but does not check for other errors.
- Provides a balance between speed and error avoidance.

#### **- Fast-Forward**

- Often a vendor-specific term for cut-through switching.
- Behavior is essentially the same as cut-through: forwards frames as soon as the destination MAC is read.
- Focuses on minimizing latency.

---

### Visual Comparison

| Feature                  | Store-and-Forward | Cut-Through   | Fragment-Free  | Fast-Forward   |
|--------------------------|-------------------|---------------|----------------|---------------|
| Latency                  | High              | Low           | Medium         | Low           |
| Error Detection (CRC)    | Yes               | No            | No (except runts) | No        |
| Filters Runts            | Yes               | No            | Yes            | No            |
| When Forwarding Starts   | After full frame  | After 6–8 bytes | After 64 bytes | After 6–8 bytes |
| Port Speed Conversion    | Yes               | No            | No             | No            |

---

**In summary:**  

- **Store-and-forward** maximizes reliability but adds delay.
- **Cut-through** (and fast-forward) minimizes delay but may propagate errors.
- **Fragment-free** offers a compromise, filtering out the most common collision fragments but not all errors.

## Ethernet Frame Fields

An Ethernet frame is a structured packet used for data transmission at the data link layer. It encapsulates both addressing and error-checking information, along with the payload. Here are the main fields in a standard Ethernet II frame:

---

### Ethernet Frame Structure

| Field                    | Size (Bytes) | Description                                                                 |
|--------------------------|--------------|-----------------------------------------------------------------------------|
| Preamble                 | 7            | Synchronizes the receiver's clock; pattern of alternating 1s and 0s         |
| Start Frame Delimiter    | 1            | Marks the end of preamble and start of the frame (10101011)                 |
| Destination MAC Address  | 6            | MAC address of the receiving device                                         |
| Source MAC Address       | 6            | MAC address of the sending device                                           |
| EtherType/Length         | 2            | Indicates protocol type (e.g., IPv4, IPv6) or payload length                |
| (Optional) VLAN Tag      | 4            | Used for VLAN identification (802.1Q)                                       |
| Payload (Data and Pad)   | 46–1500      | Data being transmitted; padded if less than 46 bytes                        |
| Frame Check Sequence     | 4            | CRC for error checking                                                      |

---

### Field Descriptions

- **Preamble:** 7 bytes of alternating 1s and 0s for synchronization.
- **Start Frame Delimiter (SFD):** 1 byte (10101011) indicating the start of the actual frame.
- **Destination MAC Address:** 6 bytes specifying the intended recipient.
- **Source MAC Address:** 6 bytes specifying the sender.
- **EtherType/Length:** 2 bytes; EtherType (≥1536) specifies the upper-layer protocol, or Length (≤1500) for payload size.
- **VLAN Tag (optional):** 4 bytes inserted after the source MAC for VLAN tagging.
- **Payload (Data and Pad):** Actual data; minimum 46 bytes (padded if needed), maximum 1500 bytes.
- **Frame Check Sequence (FCS):** 4 bytes CRC for error detection.

---

### Visual Representation

| Preamble | SFD | Dest MAC | Src MAC | EtherType/Length | [VLAN Tag] | Payload (Data+Pad) | FCS |
| 7 B | 1 B | 6 B | 6 B | 2 B | 4 B | 46–1500 B | 4 B |

---

**Note:**  

- VLAN Tag is optional and present only in VLAN-enabled networks.
- The minimum Ethernet frame size is 64 bytes (including all fields).
- The maximum payload is 1500 bytes, and the maximum frame size (without VLAN) is 1518 bytes.

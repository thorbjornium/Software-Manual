# Single VLAN for Wi-Fi

Add a single VLAN to an existing network on Wi-Fi.

Device: hAP ax<sup>3</sup>
Software: RouterOS 7

## Gemini

```routeros

# Configuration for adding a new VLAN and SSID for "pax" network

/interface bridge
add name=bridge-pax protocol-mode=none

/interface ethernet

# Assuming ether1 is your WAN interface. Adjust if your WAN is different

# If you have multiple LAN ports, identify the one connected to your Wireless Access Point (WAP) or the one you want to use for the VLAN trunk

# For simplicity, let's assume your main LAN is on a bridge or directly on an ether port

# If your WAP is connected to a specific LAN port (e.g., ether2), you might need to add ether2 to a bridge or use it directly for VLAN

# For this example, we'll assume a WAP connected to a port that's part of your existing bridge or a dedicated port

# If your WAP is connected to an existing bridge (e.g., bridge-local), you'd define the VLAN interface on that bridge

/interface vlan
add interface=bridge-local name=vlan50 vlan-id=50 # Assuming bridge-local is your main LAN bridge

/ip address
add address=192.168.50.1/24 interface=vlan50 network=192.168.50.0

/ip pool
add name=dhcp-pool-pax ranges=192.168.50.100-192.168.50.200

/ip dhcp-server
add address-pool=dhcp-pool-pax interface=vlan50 name=dhcp-pax
/ip dhcp-server network
add address=192.168.50.0/24 dns-server=8.8.8.8,8.8.4.4 gateway=192.168.50.1

/interface wireless

# This section assumes you have a Wireless interface configured, e.g., wlan1

# If your MikroTik is also your Access Point

# Add a virtual AP interface for the new SSID

add master-interface=wlan1 name=wlan-pax ssid=pax security-profile=pax-profile \
    vlan-id=50 vlan-mode=use-tag-if-present

/interface wireless security-profiles
add authentication-types=wpa2-psk mode=dynamic-keys name=pax-profile \
    supplicant-identity="" wpa2-pre-shared-key="YourStrongPasswordHere" # !!! CHANGE THIS PASSWORD !!!

# Firewall Rules for Isolation and NAT

/ip firewall filter

# Drop traffic from pax network to 192.168.88.0/24

add action=drop chain=forward in-interface=vlan50 dst-address=192.168.88.0/24 \
    comment="Block pax to 192.168.88.0"

# Drop traffic from pax network to 192.168.20.0/24

add action=drop chain=forward in-interface=vlan50 dst-address=192.168.20.0/24 \
    comment="Block pax to 192.168.20.0"

# --- Isolation from other networks (including future ones) ---

# This rule drops all traffic from vlan50 to RFC1918 private ranges (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)

# except for the vlan50 network itself

# This is a more robust way to isolate from *all* other internal networks

add action=drop chain=forward in-interface=vlan50 dst-address-list=private_networks \
    src-address=!192.168.50.0/24 comment="Isolate pax from other private networks"

/ip firewall address-list
add address=10.0.0.0/8 list=private_networks
add address=172.16.0.0/12 list=private_networks
add address=192.168.0.0/16 list=private_networks

# Allow established/related connections from pax network

add action=accept chain=forward connection-state=established,related in-interface=vlan50 \
    comment="Allow established/related from pax"

# Allow pax network to access the internet (assuming your WAN is ether1)

add action=accept chain=forward in-interface=vlan50 out-interface=ether1 \
    comment="Allow pax to internet"

# NAT for the new pax network

/ip firewall nat
add action=masquerade chain=srcnat out-interface=ether1 src-address=192.168.50.0/24 \
    comment="NAT for pax network"

# End of Configuration

```

## Deepseek

```routeros

# Configuration for adding a new VLAN and SSID for "pax" network

/interface bridge
add name=bridge-pax protocol-mode=none

/interface ethernet
# Assuming ether1 is your WAN interface. Adjust if your WAN is different.
# If you have multiple LAN ports, identify the one connected to your Wireless Access Point (WAP) or the one you want to use for the VLAN trunk.
# For simplicity, let's assume your main LAN is on a bridge or directly on an ether port.
# If your WAP is connected to a specific LAN port (e.g., ether2), you might need to add ether2 to a bridge or use it directly for VLAN.
# For this example, we'll assume a WAP connected to a port that's part of your existing bridge or a dedicated port.
# If your WAP is connected to an existing bridge (e.g., bridge-local), you'd define the VLAN interface on that bridge.

/interface vlan
add interface=bridge-local name=vlan50 vlan-id=50 # Assuming bridge-local is your main LAN bridge

/ip address
add address=192.168.50.1/24 interface=vlan50 network=192.168.50.0

/ip pool
add name=dhcp-pool-pax ranges=192.168.50.100-192.168.50.200

/ip dhcp-server
add address-pool=dhcp-pool-pax interface=vlan50 name=dhcp-pax
/ip dhcp-server network
add address=192.168.50.0/24 dns-server=8.8.8.8,8.8.4.4 gateway=192.168.50.1

/interface wireless
# This section assumes you have a Wireless interface configured, e.g., wlan1
# If your MikroTik is also your Access Point:
# Add a virtual AP interface for the new SSID
add master-interface=wlan1 name=wlan-pax ssid=pax security-profile=pax-profile \
    vlan-id=50 vlan-mode=use-tag-if-present

/interface wireless security-profiles
add authentication-types=wpa2-psk mode=dynamic-keys name=pax-profile \
    supplicant-identity="" wpa2-pre-shared-key="YourStrongPasswordHere" # !!! CHANGE THIS PASSWORD !!!

# Firewall Rules for Isolation and NAT

/ip firewall filter
# Drop traffic from pax network to 192.168.88.0/24
add action=drop chain=forward in-interface=vlan50 dst-address=192.168.88.0/24 \
    comment="Block pax to 192.168.88.0"
# Drop traffic from pax network to 192.168.20.0/24
add action=drop chain=forward in-interface=vlan50 dst-address=192.168.20.0/24 \
    comment="Block pax to 192.168.20.0"

# --- Isolation from other networks (including future ones) ---
# This rule drops all traffic from vlan50 to RFC1918 private ranges (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16)
# except for the vlan50 network itself.
# This is a more robust way to isolate from *all* other internal networks.
add action=drop chain=forward in-interface=vlan50 dst-address-list=private_networks \
    src-address=!192.168.50.0/24 comment="Isolate pax from other private networks"

/ip firewall address-list
add address=10.0.0.0/8 list=private_networks
add address=172.16.0.0/12 list=private_networks
add address=192.168.0.0/16 list=private_networks

# Allow established/related connections from pax network
add action=accept chain=forward connection-state=established,related in-interface=vlan50 \
    comment="Allow established/related from pax"

# Allow pax network to access the internet (assuming your WAN is ether1)
add action=accept chain=forward in-interface=vlan50 out-interface=ether1 \
    comment="Allow pax to internet"

# NAT for the new pax network
/ip firewall nat
add action=masquerade chain=srcnat out-interface=ether1 src-address=192.168.50.0/24 \
    comment="NAT for pax network"

# End of Configuration

```

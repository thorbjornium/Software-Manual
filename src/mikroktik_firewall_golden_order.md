# Golden Order

<!-- toc -->

## INPUT chain (traffic to the router itself)  

```routeros

/ip firewall filter  
add chain=input action=drop connection-state=invalid comment="Drop invalid"  
add chain=input action=accept connection-state=established,related,untracked comment="Accept established/related/untracked"  

```

## Allow specific services you need (WireGuard, SSH, WinBox, HTTPS, ICMP from trusted LANs)

```routeros

add chain=input action=accept in-interface-list=LAN_Trusted_Interfaces comment="Allow LAN management"
add chain=input action=accept dst-address=127.0.0.1 comment="Allow loopback"
add chain=input action=accept ipsec-policy=in,ipsec comment="Allow IPsec in"
add chain=input action=drop in-interface-list=!LAN comment="Drop all not from LAN"

```

## FORWARD chain (traffic passing through the router)  

```routeros

/ip firewall filter
add chain=forward action=drop connection-state=invalid comment="Drop invalid"
add chain=forward action=fasttrack-connection hw-offload=yes connection-state=established,related comment="Fasttrack established/related"
add chain=forward action=accept connection-state=established,related,untracked comment="Accept established/related/untracked"
add chain=forward action=accept ipsec-policy=in,ipsec comment="Allow IPsec in"
add chain=forward action=accept ipsec-policy=out,ipsec comment="Allow IPsec out"

```

## Allow your VPN rules (WireGuard, back-to-home, etc.)

## Allow LAN <-> VPN

## Allow specific port forwards (e.g. qBittorrent)

## Special drops (multicast, isolated WiFi subnets, inter-VLAN blocking)

```routeros

add chain=forward action=drop connection-state=new connection-nat-state=!dstnat in-interface-list=WAN comment="Drop all from WAN not dstnat"

```

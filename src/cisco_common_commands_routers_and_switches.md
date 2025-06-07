# Common Cisco IOS Commands for Routers and Switches

<!-- toc -->

This list covers fundamental and frequently used commands.

## Navigation and Basic System Commands

* `Router>`: User EXEC mode prompt (limited commands).
* `Switch>`: User EXEC mode prompt (limited commands).
* `Router#`: Privileged EXEC mode prompt (all monitoring commands, allows entry to config modes).
* `Switch#`: Privileged EXEC mode prompt (all monitoring commands, allows entry to config modes).
* `Router(config)#`: Global Configuration mode prompt.
* `Switch(config)#`: Global Configuration mode prompt.

* `enable`: Enter privileged EXEC mode from user EXEC mode.
* `disable`: Exit privileged EXEC mode to user EXEC mode.
* `configure terminal` (or `config t`): Enter global configuration mode.
* `exit`: Exit the current configuration mode or log out from user EXEC mode.
* `end`: Exit all configuration modes and return to privileged EXEC mode.
* `logout`: Log out from the current session.
* `?`: Displays context-sensitive help for commands.
* `<command> ?`: Displays arguments for a command.
* `<command> <partial-argument>?`: Displays options for a partial argument.
* `<command>` (tab): Autocompletes a command.

## Basic Device Configuration

* `hostname <name>`: Sets the hostname of the device (Global config).
* `enable secret <password>`: Sets an encrypted privileged EXEC mode password (Global config).
* `enable password <password>`: Sets a clear-text privileged EXEC mode password (less secure, Global config).
* `service password-encryption`: Encrypts all clear-text passwords in the configuration (Global config).
* `banner motd #<text>#`: Configures a Message Of The Day banner (Global config). The `#` can be any character not in the text.
* `line console 0`: Enters console line configuration mode (Global config).
* `password <password>`: Sets a password for the line (Line config).
* `login`: Requires password authentication for the line (Line config).
* `exec-timeout <minutes> <seconds>`: Sets the inactivity timeout for the line (Line config).
* `logging synchronous`: Prevents console messages from interrupting command input (Line config).
* `line vty 0 4`: Enters virtual terminal (Telnet/SSH) line configuration mode (Global config). `0 4` typically means 5 VTY lines.

## Interface Configuration

* `interface <type> <number>` (e.g., `interface GigabitEthernet0/1`, `interface FastEthernet0/0`): Enters interface configuration mode (Global config).
* `ip address <ip-address> <subnet-mask>`: Assigns an IP address and subnet mask to the interface (Interface config).
* `no shutdown`: Activates the interface (Interface config).
* `shutdown`: Deactivates the interface (Interface config).
* `description <text>`: Adds a description to the interface (Interface config).
* `ipv6 address <ipv6-address>/<prefix-length>`: Assigns an IPv6 address to the interface (Interface config).
* `ipv6 address autoconfig`: Enables IPv6 stateless autoconfiguration (SLAAC) on the interface (Interface config).
* `duplex <auto|full|half>`: Sets the duplex mode (Interface config).
* `speed <auto|10|100|1000>`: Sets the interface speed (Interface config).

## VLANs (Switches Only)

* `vlan <vlan-id>`: Creates a VLAN and enters VLAN configuration mode (Global config).
* `name <vlan-name>`: Assigns a name to the VLAN (VLAN config).
* `interface <type> <number>`: Selects an interface (Global config).
* `switchport mode access`: Configures the interface as an access port (Interface config).
* `switchport access vlan <vlan-id>`: Assigns the access port to a specific VLAN (Interface config).
* `switchport mode trunk`: Configures the interface as a trunk port (Interface config).
* `switchport trunk encapsulation <dot1q|isl>`: Specifies the trunking encapsulation (Interface config).
* `switchport trunk allowed vlan <vlan-list>`: Allows specific VLANs on the trunk (Interface config).
* `switchport trunk native vlan <vlan-id>`: Sets the native VLAN for the trunk (Interface config).
* `no vlan <vlan-id>`: Deletes a VLAN (Global config).

## Routing (Routers Primarily)

### Static Routing

* `ip route <destination-network> <subnet-mask> <next-hop-ip | exit-interface>`: Configures a static route (Global config).
* `ip route 0.0.0.0 0.0.0.0 <next-hop-ip | exit-interface>`: Configures a default route (Global config).
* `no ip route <destination-network> <subnet-mask> <next-hop-ip | exit-interface>`: Removes a static route (Global config).

### OSPF (Open Shortest Path First)

* `router ospf <process-id>`: Enables OSPF routing process (Global config).
* `network <network-address> <wildcard-mask> area <area-id>`: Specifies networks to be included in the OSPF process and their area (Router config).
* `passive-interface <interface-type> <number>`: Prevents OSPF updates from being sent out a specific interface (Router config).

### EIGRP (Enhanced Interior Gateway Routing Protocol)

* `router eigrp <asn>`: Enables EIGRP routing process (Global config). `asn` is the Autonomous System Number.
* `network <network-address>`: Specifies networks to be included in the EIGRP process (Router config).
* `no auto-summary`: Disables automatic summarization of networks (Router config).

## Security

### Access Control Lists (ACLs)

* `access-list <acl-number> <permit|deny> <source-ip> <source-wildcard>` (Standard ACL): Creates a standard IP ACL (Global config).
* `access-list <acl-number> <permit|deny> <protocol> <source-ip> <source-wildcard> <destination-ip> <destination-wildcard> [eq|gt|lt|neq] <port>` (Extended ACL): Creates an extended IP ACL (Global config).
* `ip access-group <acl-number> <in|out>`: Applies an ACL to an interface (Interface config).
* `ip access-list standard <acl-name>`: Creates a named standard ACL (Global config).
* `ip access-list extended <acl-name>`: Creates a named extended ACL (Global config).
* `permit|deny <conditions>`: Adds permit/deny statements within a named ACL (Named ACL config).

### Port Security (Switches Only)

* `interface <type> <number>`: Selects an interface (Global config).
* `switchport mode access`: Configures the interface as an access port (Interface config).
* `switchport port-security`: Enables port security on the interface (Interface config).
* `switchport port-security maximum <number>`: Sets the maximum number of secure MAC addresses (Interface config).
* `switchport port-security mac-address sticky`: Enables sticky learning of MAC addresses (Interface config).
* `switchport port-security violation <shutdown|restrict|protect>`: Sets the violation mode (Interface config).

### SSH Configuration

* `ip domain-name <domain-name>`: Configures the domain name (Global config).
* `crypto key generate rsa`: Generates RSA keys for SSH (Global config).
* `username <username> secret <password>`: Creates a local user (Global config).
* `line vty 0 4`: Enters VTY line configuration mode (Global config).
* `transport input ssh`: Allows only SSH connections on VTY lines (Line config).
* `login local`: Requires local authentication (Line config).

## Monitoring and Verification (Show Commands)

These commands are typically run from Privileged EXEC mode (`#`).

* `show version`: Displays system hardware, software version, and uptime.
* `show running-config` (or `sh run`): Displays the current active configuration in RAM.
* `show startup-config` (or `sh start`): Displays the configuration saved in NVRAM (loaded at boot).
* `show ip interface brief` (or `sh ip int brie`): Displays a summary of interface IP addresses and status (up/down).
* `show interfaces <type> <number>`: Displays detailed status and statistics for a specific interface.
* `show ip route`: Displays the IP routing table.
* `show cdp neighbors detail`: Displays detailed information about directly connected Cisco devices.
* `show mac address-table` (or `show mac address-table dynamic`): Displays the MAC address table on a switch.
* `show vlan brief`: Displays a summary of VLANs and their assigned ports.
* `show interfaces trunk`: Displays trunk port status and allowed VLANs.
* `show ip protocols`: Displays information about active routing protocols.
* `show controllers <type> <number>`: Displays hardware-specific information for an interface.
* `show log`: Displays system log messages.
* `show users`: Displays current console and VTY users.
* `show history`: Displays previously entered commands.
* `show flash:`: Displays contents of flash memory.

## Troubleshooting Commands

* `ping <ip-address|hostname>`: Tests IP connectivity.
* `traceroute <ip-address|hostname>`: Traces the path packets take to a destination.
* `telnet <ip-address|hostname>`: Opens a Telnet connection to another device.
* `debug <feature>`: Enables debugging output for a specific feature (use with caution, can impact performance).
* `no debug all` or `undebug all` (or `u all`): Disables all debugging.
* `clear ip route *`: Clears the IP routing table (use with caution).
* `clear mac address-table dynamic`: Clears dynamically learned MAC addresses.
* `clear line <line-number>`: Clears a VTY or console line.

## Configuration Management

* `copy running-config startup-config` (or `wr mem` or `copy run start`): Saves the running configuration to NVRAM.
* `copy running-config tftp:`: Copies the running configuration to a TFTP server.
* `copy tftp: running-config`: Copies a configuration file from a TFTP server to running-config.
* `erase startup-config`: Erases the startup configuration from NVRAM.
* `reload`: Reboots the device.
* `archive log config`: Configures logging of configuration changes.
* `show archive log config all`: Shows the history of configuration changes.

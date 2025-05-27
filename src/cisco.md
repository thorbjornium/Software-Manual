# Cisco Router & Switch Configuration  

<!-- toc -->

## **Part 1: Configuring a Cisco Router**

Routers operate at Layer 3 (Network Layer) and are responsible for forwarding packets between different networks (IP subnets).

**Assumptions:**

* You have a Cisco router (physical or in Packet Tracer/GNS3).
* You have a console cable connected from your PC's serial port (or USB-to-serial adapter) to the router's console port.
* You are using a terminal emulation program (like PuTTY, Tera Term, or Packet Tracer's Terminal app) with settings typically `9600 baud, 8 data bits, no parity, 1 stop bit, no flow control`.

---

### 1. Access the Router CLI

* Power on the router.
* Open your terminal emulation program and connect to the console port.
* You'll likely see a boot-up sequence. If it asks "Would you like to enter the initial configuration dialog? [yes/no]:", type `no` and press Enter.
* You'll arrive at the **User EXEC mode**: `Router>`

### 2. Enter Privileged EXEC Mode

* From `Router>`, type `enable` or `en`.
* Press Enter.
* The prompt changes to `Router#`. This mode allows you to view system information and enter global configuration mode.

### 3. Enter Global Configuration Mode

* From `Router#`, type `configure terminal` or `conf t`.
* Press Enter.
* The prompt changes to `Router(config)#`. This mode allows you to make system-wide changes.

### 4. Basic Router Configuration

* **Set a Hostname:**

    ```
    Router(config)# hostname MyBranchRouter
    MyBranchRouter(config)#
    ```

    *(The prompt changes immediately)*

* **Secure Privileged EXEC Mode (Encrypts password):**

    ```
    MyBranchRouter(config)# enable secret YourStrongPassword
    ```

    *(This password will be requested when typing `enable`)*

* **Secure Console Line (Local Login for Console):**

    ```
    MyBranchRouter(config)# line console 0
    MyBranchRouter(config-line)# password ConsolePass
    MyBranchRouter(config-line)# login
    MyBranchRouter(config-line)# exit
    ```

    *(This password will be requested when connecting via console)*

* **Secure VTY Lines (for Telnet/SSH access):**
    VTY lines allow remote access. A common range is `0 4` (allowing 5 simultaneous sessions).

    ```
    MyBranchRouter(config)# line vty 0 4
    MyBranchRouter(config-line)# password VTYPass
    MyBranchRouter(config-line)# login
    MyBranchRouter(config-line)# exit
    ```

    *(These passwords will be requested when connecting via Telnet/SSH)*

* **Encrypt all plaintext passwords:**

    ```
    MyBranchRouter(config)# service password-encryption
    ```

    *(This encrypts passwords set using `password` command, not `enable secret` which is already hashed)*

* **Set a Banner (Message of the Day):**

    ```
    MyBranchRouter(config)# banner motd # Unauthorized access is prohibited #
    ```

    *(The `#` is a delimiter; you can use any character not in the message)*

### 5. Configure Router Interfaces (Assign IP Addresses)

Routers have physical interfaces (e.g., GigabitEthernet, FastEthernet, Serial). Each interface usually connects to a different network segment.

* **Identify Interfaces:** In Privileged EXEC mode (`MyBranchRouter#`), you can use:

    ```
    show ip interface brief
    ```

    This shows all interfaces, their IP addresses (if any), and their status.

* **Enter Interface Configuration Mode:**

    ```
    MyBranchRouter(config)# interface GigabitEthernet0/0  (or FastEthernet0/0, or Serial0/0/0 etc.)
    MyBranchRouter(config-if)#
    ```

* **Assign IP Address and Subnet Mask:**

    ```
    MyBranchRouter(config-if)# ip address 192.168.1.1 255.255.255.0
    ```

* **Activate the Interface:** By default, interfaces are administratively down.

    ```
    MyBranchRouter(config-if)# no shutdown
    ```

    *(You should see a message about the interface changing state to up)*

* **Exit Interface Mode:**

    ```
    MyBranchRouter(config-if)# exit
    MyBranchRouter(config)#
    ```

    *(Repeat for all interfaces you need to configure)*

### 6. Configure a Default Route (for Internet Access)

If your router connects to the internet (or another router that leads to the internet), you'll need a default route. This tells the router where to send packets if it doesn't have a specific route for the destination network.

* `0.0.0.0 0.0.0.0` represents "any network, any subnet mask."
* `10.0.0.1` would be the IP address of the "next-hop" router (e.g., your ISP's router or your main home router).

    ```
    MyBranchRouter(config)# ip route 0.0.0.0 0.0.0.0 10.0.0.1
    ```

### 7. Save the Configuration

Changes made in `config` modes are only in the running-configuration (RAM). To save them permanently so they survive a reboot, you need to copy them to startup-configuration (NVRAM).

* From `MyBranchRouter(config)#`, type `exit` to go back to Privileged EXEC mode.

    ```
    MyBranchRouter(config)# exit
    MyBranchRouter#
    ```

* Save the configuration:

    ```
    MyBranchRouter# copy running-config startup-config
    Destination filename [startup-config]? (Press Enter)
    Building configuration...
    ```

### 8. Verify Configuration (from Privileged EXEC mode #)

* `show running-config`: Shows the current active configuration in RAM.
* `show ip interface brief`: Shows a summary of all interfaces, their IP addresses, and status.
* `show ip route`: Shows the routing table.
* `ping <destination_ip>`: Test connectivity (e.g., `ping 192.168.1.254`).

---

## **Part 2: Configuring a Cisco Switch**

Switches primarily operate at Layer 2 (Data Link Layer) and are responsible for forwarding frames within the same local network (LAN) based on MAC addresses.

**Assumptions:**

* You have a Cisco switch (physical or in Packet Tracer/GNS3).
* You have a console cable connected from your PC's serial port to the switch's console port.
* You are using a terminal emulation program (like PuTTY, Tera Term, or Packet Tracer's Terminal app).

---

### 1. Access the Switch CLI

* Power on the switch.
* Open your terminal emulation program and connect to the console port.
* You'll likely see a boot-up sequence.
* You'll arrive at the **User EXEC mode**: `Switch>`

### 2. Enter Privileged EXEC Mode

* From `Switch>`, type `enable` or `en`.
* Press Enter.
* The prompt changes to `Switch#`.

### 3. Enter Global Configuration Mode

* From `Switch#`, type `configure terminal` or `conf t`.
* Press Enter.
* The prompt changes to `Switch(config)#`.

### 4. Basic Switch Configuration (Similar to Router)

* **Set a Hostname:**

    ```
    Switch(config)# hostname MyFloorSwitch
    MyFloorSwitch(config)#
    ```

* **Secure Privileged EXEC Mode:**

    ```
    MyFloorSwitch(config)# enable secret YourStrongPassword
    ```

* **Secure Console Line:**

    ```
    MyFloorSwitch(config)# line console 0
    MyFloorSwitch(config-line)# password ConsolePass
    MyFloorSwitch(config-line)# login
    MyFloorSwitch(config-line)# exit
    ```

* **Secure VTY Lines (for Telnet/SSH access):**

    ```
    MyFloorSwitch(config)# line vty 0 15  (Switches typically have more VTY lines than routers, up to 16)
    MyFloorSwitch(config-line)# password VTYPass
    MyFloorSwitch(config-line)# login
    MyFloorSwitch(config-line)# exit
    ```

* **Encrypt all plaintext passwords:**

    ```
    MyFloorSwitch(config)# service password-encryption
    ```

* **Set a Banner:**

    ```
    MyFloorSwitch(config)# banner motd # Authorized access only #
    ```

### 5. Configure VLANs (Virtual Local Area Networks)

VLANs segment a broadcast domain. By default, all ports are in VLAN 1.

* **Create a new VLAN:**

    ```
    MyFloorSwitch(config)# vlan 10
    MyFloorSwitch(config-vlan)# name Sales_VLAN
    MyFloorSwitch(config-vlan)# exit
    ```

    *(Repeat for other VLANs as needed, e.g., vlan 20 name IT_VLAN)*

### 6. Assign Access Ports to VLANs

Access ports carry traffic for a single VLAN.

* **Enter Interface Configuration Mode for the port:**

    ```
    MyFloorSwitch(config)# interface FastEthernet0/1 (or GigabitEthernet0/1 etc.)
    MyFloorSwitch(config-if)#
    ```

* **Set port mode to access:**

    ```
    MyFloorSwitch(config-if)# switchport mode access
    ```

* **Assign the port to a VLAN:**

    ```
    MyFloorSwitch(config-if)# switchport access vlan 10
    MyFloorSwitch(config-if)# exit
    ```

    *(Repeat for all access ports)*

### 7. Configure Trunk Ports (for inter-VLAN communication between switches/routers)

Trunk ports carry traffic for multiple VLANs, typically connecting switches to other switches or to routers (for inter-VLAN routing).

* **Enter Interface Configuration Mode for the trunk port:**

    ```
    MyFloorSwitch(config)# interface GigabitEthernet0/1  (Commonly use GigE for trunks)
    MyFloorSwitch(config-if)#
    ```

* **Set encapsulation type (usually Dot1Q):**

    ```
    MyFloorSwitch(config-if)# switchport trunk encapsulation dot1q
    ```

* **Set port mode to trunk:**

    ```
    MyFloorSwitch(config-if)# switchport mode trunk
    ```

* **Allow specific VLANs on the trunk (optional, good practice):**

    ```
    MyFloorSwitch(config-if)# switchport trunk allowed vlan 1,10,20
    ```

    *(VLAN 1 is the default and usually remains allowed)*

* **Exit Interface Mode:**

    ```
    MyFloorSwitch(config-if)# exit
    ```

### 8. Configure a Management IP Address (Switched Virtual Interface - SVI)

Switches are Layer 2 devices, but for remote management (Telnet/SSH) they need an IP address. This is done by creating a Switched Virtual Interface (SVI) for a VLAN.

* **Enter Interface VLAN mode:**

    ```
    MyFloorSwitch(config)# interface vlan 1  (VLAN 1 is the default management VLAN)
    MyFloorSwitch(config-if)#
    ```

* **Assign IP Address and Subnet Mask:**

    ```
    MyFloorSwitch(config-if)# ip address 192.168.1.254 255.255.255.0
    ```

* **Activate the Interface:**

    ```
    MyFloorSwitch(config-if)# no shutdown
    ```

    *(You should see a message about the interface changing state to up)*

* **Exit Interface Mode:**

    ```
    MyFloorSwitch(config-if)# exit
    ```

### 9. Configure a Default Gateway (for management to reach other subnets)

For the switch to be managed from another network (e.g., from a PC on a different VLAN or the internet), it needs a default gateway (the IP address of the router connected to that VLAN).

* ```
    MyFloorSwitch(config)# ip default-gateway 192.168.1.1
    ```

### 10. Save the Configuration

* From `MyFloorSwitch(config)#`, type `exit` to go back to Privileged EXEC mode.

    ```
    MyFloorSwitch(config)# exit
    MyFloorSwitch#
    ```

* Save the configuration:

    ```
    MyFloorSwitch# copy running-config startup-config
    Destination filename [startup-config]? (Press Enter)
    Building configuration...
    ```

### 11. Verify Configuration (from Privileged EXEC mode #)

* `show running-config`: Shows the current active configuration in RAM.
* `show ip interface brief`: Shows a summary of VLAN interfaces and their IP addresses.
* `show vlan brief`: Shows a summary of all VLANs and port assignments.
* `show mac address-table`: Shows the MAC address table.
* `ping <ip_address>`: Test connectivity (e.g., `ping 192.168.1.1`).

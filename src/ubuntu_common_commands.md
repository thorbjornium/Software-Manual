# List of Common Ubuntu Commands

<!-- toc -->

## File and Directory Management

- `ls` - List directory contents
- `ls -a` - List all files, including hidden files
- `ls -lh` - List files with human-readable sizes
- `ls -R` - List files in directories and subdirectories
- `cd [directory]` - Change the current directory
- `cd ..` - Move up one directory
- `cd -` - Switch to the previous directory
- `cd ~` - Change to the home directory
- `pwd` - Print the current working directory
- `mkdir [directory]` - Create a new directory
- `mkdir -p [path/to/directory]` - Create parent directories as needed
- `rmdir [directory]` - Remove an empty directory
- `rm [file]` - Remove a file
- `rm -r [directory]` - Remove a directory and its contents recursively
- `rm -f [file]` - Force remove a file without confirmation
- `cp [source] [destination]` - Copy files and directories
- `cp -r [source] [destination]` - Copy directories recursively
- `mv [source] [destination]` - Move or rename files and directories
- `touch [file]` - Create an empty file or update the timestamp of a file
- `cat [file]` - Concatenate and display file content
- `head [file]` - Display the first 10 lines of a file
- `tail [file]` - Display the last 10 lines of a file
- `tail -f [file]` - Follow the end of a file in real-time
- `find [path] -name [filename]` - Search for files by name
- `find [path] -type f -name '*.txt'` - Find all text files
- `file [filename]` - Determine the file type
- `ln -s [target] [link]` - Create a symbolic link
- `du -sh [directory]` - Display disk usage of a directory in human-readable format
- `stat [file]` - Display file or file system status
- `tree` - Display a directory structure in a tree format (requires installation)
- `xargs` - Build and execute command lines from standard input

## System Information

- `uname -a` - Display system information
- `top` - Display active processes
- `htop` - Interactive process viewer (requires installation)
- `df -h` - Show disk space usage
- `free -h` - Show memory usage
- `uptime` - Show how long the system has been running
- `lscpu` - Display CPU architecture information
- `lsblk` - List block devices
- `dmesg` - Show kernel ring buffer messages
- `ps aux` - Display all running processes
- `whoami` - Show the current user
- `last` - Show last logged in users
- `cat /proc/cpuinfo` - Display CPU information
- `cat /proc/meminfo` - Display memory information
- `vmstat` - Report information about processes, memory, paging, block I/O, traps, and CPU activity
- `lsof` - List open files and the processes that opened them
- `uptime` - Show system load averages and uptime
- `hostname` - Display the system's hostname

## Package Management

- `sudo apt update` - Update package lists
- `sudo apt upgrade` - Upgrade installed packages
- `sudo apt dist-upgrade` - Upgrade packages, handling dependencies
- `sudo apt install [package]` - Install a package
- `sudo apt remove [package]` - Remove a package
- `apt search [keyword]` - Search for a package
- `sudo apt autoremove` - Remove unnecessary packages
- `apt list --installed` - List all installed packages
- `apt show [package]` - Display information about a package
- `apt-cache policy [package]` - Show package version information
- `dpkg -l` - List all installed packages using dpkg
- `sudo apt purge [package]` - Remove a package and its configuration files
- `sudo apt-get clean` - Clear local repository of retrieved package files
- `sudo dpkg -i [package.deb]` - Install a .deb package
- `sudo apt-mark hold [package]` - Prevent a package from being upgraded
- `apt-file search [filename]` - Search for a file in available packages (requires installation)
- `sudo apt edit-sources` - Edit the sources list for package management

## User Management

- `adduser [username]` - Add a new user
- `deluser [username]` - Delete a user
- `passwd [username]` - Change a user's password
- `groups [username]` - List groups a user belongs to
- `who` - Show who is logged in
- `last` - Show last logged in users
- `usermod -aG [group] [username]` - Add a user to a group
- `sudo su - [username]` - Switch to another user
- `finger [username]` - Show information about a user
- `chage -l [username]` - Show password aging information
- `sudo visudo` - Edit the sudoers file
- `userdel -r [username]` - Delete a user and their home directory
- `id [username]` - Show user ID and group ID information
- `newgrp [group]` - Log in to a new group
- `groups` - List groups for the current user
- `sudo usermod -d [new_home] [username]` - Change a user’s home directory

## Networking

- `ifconfig` - Display network interfaces (deprecated; use `ip addr`)
- `ip addr` - Show IP address and network details
- `ping [host]` - Check connectivity to a host
- `curl [URL]` - Transfer data from or to a server
- `wget [URL]` - Download files from the web
- `netstat -tuln` - Display listening ports
- `traceroute [host]` - Trace the route to a network host
- `nslookup [domain]` - Query DNS for a domain
- `dig [domain]` - Perform DNS lookup (requires installation)
- `whois [domain]` - Retrieve domain registration information
- `ssh [user]@[host]` - Securely connect to a remote server
- `scp [file] [user]@[host]:[path]` - Securely copy files to a remote server
- `ftp [host]` - Connect to an FTP server
- `telnet [host] [port]` - Connect to a host via Telnet (if installed)
- `route -n` - Display the routing table
- `ip link show` - Show all network interfaces
- `nmap [host]` - Network exploration tool and security/port scanner (requires installation)

## Disk Management

- `mount` - List mounted filesystems
- `umount [device]` - Unmount a filesystem
- `fdisk -l` - List disk partitions
- `df -i` - Show inode usage
- `du -h [directory]` - Show directory size in human-readable format
- `lsblk` - List block devices with mount points
- `badblocks [device]` - Check for bad sectors on a disk
- `fsck [device]` - Check and repair a filesystem
- `mkfs.ext4 [device]` - Create an ext4 filesystem on a device
- `resize2fs [device]` - Resize an ext2/ext3/ext4 filesystem
- `dd if=[source] of=[destination]` - Copy and convert files (use with caution)
- `parted [device]` - Manage disk partitions interactively
- `mount -o loop [file] [mountpoint]` - Mount an image file
- `pvcreate [device]` - Initialize a physical volume for LVM
- `vgcreate [volume-group] [device]` - Create a volume group in LVM

## Search and Find

- `find [path] -name [filename]` - Search for files by name
- `grep [pattern] [file]` - Search for a pattern in a file
- `locate [filename]` - Find files by name quickly
- `which [command]` - Show the full path of a command
- `whereis [command]` - Locate the binary, source, and manual page files
- `history` - Show command history
- `echo [text]` - Print text to the terminal
- `sed 's/[old]/[new]/g' [file]` - Replace text in a file
- `awk '{print $1}' [file]` - Print the first column of a file
- `grep -r [pattern] [directory]` - Recursively search for a pattern in a directory
- `find [path] -type f -exec [command] {} \;` - Execute a command on found files
- `xargs` - Build and execute command lines from standard input
- `fuzzyfind [pattern]` - Fuzzy search for files (requires installation)

## Miscellaneous

- `clear` - Clear the terminal screen
- `man [command]` - Display the manual for a command
- `alias [name]='[command]'` - Create an alias for a command
- `unalias [name]` - Remove an alias
- `exit` - Exit the terminal session
- `chmod [permissions] [file]` - Change file permissions
- `chown [user]:[group] [file]` - Change file owner and group
- `tar -cvf [archive.tar] [directory]` - Create a tar archive
- `tar -xvf [archive.tar]` - Extract a tar archive
- `zip [archive.zip] [file]` - Create a zip archive
- `unzip [archive.zip]` - Extract a zip archive
- `date` - Display the current date and time
- `cal` - Display a calendar
- `export [variable]=[value]` - Set an environment variable
- `env` - Display environment variables
- `echo $PATH` - Display the current PATH variable
- `crontab -e` - Edit the cron jobs for the current user

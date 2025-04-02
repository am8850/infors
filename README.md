# infors

A simple Rust information system not meant to be a replacement for the system tools, but as an aide to get quick information about the system.

## Installation

- Install Rust
- Clone this repo
- Type: `cargo install --path .`

## usage

```bash
Usage: infors <COMMAND>

Commands:
  memory       Shows memory usage
  cpus         Shows cpu(s) and cpu(s) usage
  network      Shows network usage
  temperature  Shows temperature
  disks        Shows disk usage
  users        Shows users
  system       Shows system information
  frequency    Shows cpu(s) frequency
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Sample runs

```bash
infors system

System name:              Ubuntu
System kernel version:    5.15.167.4-microsoft-standard-WSL2
System OS version:        24.04
System OS (long) version: Linux (Ubuntu 24.04)
System host name:         ASUSPN62S
System kernel:            Linux 5.15.167.4-microsoft-standard-WSL2

infors memory

total memory:     15.54 GiB
available memory: 8.41 GiB
used memory:      7.14 GiB
total swap:       4.00 GiB
used swap:        268.00 MiB

infors disks

"/dev/sdc" HDD "ext4" Size: 1006.85 GiB 848.50 GiB "/"
"/dev/sdc" HDD "ext4" Size: 1006.85 GiB 848.50 GiB "/mnt/wslg/distro"
"/dev/sdc" HDD "ext4" Size: 1006.85 GiB 848.50 GiB "/var/lib/docker"

infors network

eth0:
  ether fe80::215:5dff:fe64:9c25/64, 172.22.194.170/20
  ips: 00:15:5d:64:9c:25
  input data  (new / total): 0 / 385704208 B
  output data (new / total): 0 / 249739077 B
```

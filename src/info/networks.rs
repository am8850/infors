use std::io::{self, Write};
use sysinfo::Networks;

pub fn print_networks(networks: &mut Networks) {
    for (interface_name, data) in networks.iter() {
        writeln!(
            &mut io::stdout(),
            "{}:\n  ether {}\n  IPs: {}\n input data  (new / total): {} / {} B\n  output data (new / total): {} / {} B",
            interface_name,
            data.ip_networks()
                .iter()
                .map(|ip| ip.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            data.mac_address(),
            data.received(),
            data.total_received(),
            data.transmitted(),
            data.total_transmitted(),
        );
    }
}

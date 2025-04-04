use std::io::{self, Write};

use crate::info::memory::format_memory;

pub fn print_disks_info(disks: &sysinfo::Disks) {
    for disk in disks {
        if disk.kind() == sysinfo::DiskKind::HDD
            || disk.kind() == sysinfo::DiskKind::SSD
            || disk.is_removable()
        {
            writeln!(
                &mut io::stdout(),
                "{:?} {:?} {:?} Size: {} {} {:?}",
                disk.name(),
                disk.kind(),
                disk.file_system(),
                format_memory(disk.total_space()),
                format_memory(disk.available_space()),
                disk.mount_point()
            );
        } else {
            //writeln!(&mut io::stdout(), "Disk: {:?}", disk);
        }
        //writeln!(&mut io::stdout(), "{disk:?}");
    }
}

use std::io::{self, Write};

pub fn format_memory(mem: u64) -> String {
    const gb: u64 = 1024 * 1024 * 1024;
    const mb: u64 = 1024 * 1024;
    const kb: u64 = 1024;

    if mem > gb {
        format!("{:.2} GiB", mem as f64 / gb as f64)
    } else if mem > mb {
        format!("{:.2} MiB", mem as f64 / mb as f64)
    } else if mem > kb {
        format!("{:.2} MiB", mem as f64 / kb as f64)
    } else {
        format!("{} KB", mem)
    }
}

pub fn report_memory(sys: &mut sysinfo::System) {
    let stdout = &mut io::stdout();
    writeln!(
        stdout,
        "total memory:     {}",
        format_memory(sys.total_memory())
    );
    writeln!(
        &mut io::stdout(),
        "available memory: {}",
        format_memory(sys.available_memory())
    );
    writeln!(
        &mut io::stdout(),
        "used memory:      {}",
        format_memory(sys.used_memory())
    );
    writeln!(
        &mut io::stdout(),
        "total swap:       {}",
        format_memory(sys.total_swap())
    );
    writeln!(
        &mut io::stdout(),
        "used swap:        {}",
        format_memory(sys.used_swap())
    );
}

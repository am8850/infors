#![allow(unused_must_use, non_upper_case_globals)]
#![allow(clippy::manual_range_contains)]

mod disks;
mod memory;
mod networks;

use std::io::{self, Write};
use std::str::FromStr;
use sysinfo::{Components, Disks, Networks, Pid, Signal, System, Users};

const signals: &[Signal] = &[
    Signal::Hangup,
    Signal::Interrupt,
    Signal::Quit,
    Signal::Illegal,
    Signal::Trap,
    Signal::Abort,
    Signal::Bus,
    Signal::FloatingPointException,
    Signal::Kill,
    Signal::User1,
    Signal::Segv,
    Signal::User2,
    Signal::Pipe,
    Signal::Alarm,
    Signal::Term,
    Signal::Child,
    Signal::Continue,
    Signal::Stop,
    Signal::TSTP,
    Signal::TTIN,
    Signal::TTOU,
    Signal::Urgent,
    Signal::XCPU,
    Signal::XFSZ,
    Signal::VirtualAlarm,
    Signal::Profiling,
    Signal::Winch,
    Signal::IO,
    Signal::Power,
    Signal::Sys,
];

fn print_help() {}

// fn print_message(message: &str) {
//     writeln!(&mut io::stdout(), "{}", message);
// }

pub fn query(input: &str) -> bool {
    //println!("Querying: {}", input);
    println!("{}", "");
    let mut system = System::new_all();
    let mut networks = Networks::new_with_refreshed_list();
    let mut disks = Disks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();
    let mut users = Users::new_with_refreshed_list();
    let done = interpret_input(
        input.as_ref(),
        &mut system,
        &mut networks,
        &mut disks,
        &mut components,
        &mut users,
    );
    done
}

fn interpret_input(
    input: &str,
    sys: &mut System,
    networks: &mut Networks,
    disks: &mut Disks,
    components: &mut Components,
    users: &mut Users,
) -> bool {
    match input.trim() {
        "help" => print_help(),
        "refresh_all" => {
            writeln!(&mut io::stdout(), "Refreshing all...");
            sys.refresh_all();
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_disks" => {
            writeln!(&mut io::stdout(), "Refreshing disk list...");
            disks.refresh(true);
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_users" => {
            writeln!(&mut io::stdout(), "Refreshing user list...");
            users.refresh();
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_networks" => {
            writeln!(&mut io::stdout(), "Refreshing network list...");
            networks.refresh(true);
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_components" => {
            writeln!(&mut io::stdout(), "Refreshing component list...");
            components.refresh(true);
            writeln!(&mut io::stdout(), "Done.");
        }
        "refresh_cpu" => {
            writeln!(&mut io::stdout(), "Refreshing CPUs...");
            sys.refresh_cpu_all();
            writeln!(&mut io::stdout(), "Done.");
        }
        "signals" => {
            let mut nb = 1i32;

            for sig in signals {
                writeln!(&mut io::stdout(), "{nb:2}:{sig:?}");
                nb += 1;
            }
        }
        "cpus" => {
            // Note: you should refresh a few times before using this, so that usage statistics
            // can be ascertained
            writeln!(
                &mut io::stdout(),
                "number of physical cores: {}",
                System::physical_core_count()
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "Unknown".to_owned()),
            );
            writeln!(
                &mut io::stdout(),
                "total CPU usage: {}%",
                sys.global_cpu_usage(),
            );
            for cpu in sys.cpus() {
                writeln!(&mut io::stdout(), "{cpu:?}");
            }
        }
        "memory" => {
            memory::report_memory(sys);
        }
        "quit" | "exit" => return true,
        "all" => {
            for (pid, proc_) in sys.processes() {
                writeln!(
                    &mut io::stdout(),
                    "{}:{} status={:?}",
                    pid,
                    proc_.name().to_string_lossy(),
                    proc_.status()
                );
            }
        }
        "frequency" => {
            for cpu in sys.cpus() {
                writeln!(
                    &mut io::stdout(),
                    "[{}] {} MHz",
                    cpu.name(),
                    cpu.frequency(),
                );
            }
        }
        "vendor_id" => {
            writeln!(
                &mut io::stdout(),
                "vendor ID: {}",
                sys.cpus()[0].vendor_id()
            );
        }
        "brand" => {
            writeln!(&mut io::stdout(), "brand: {}", sys.cpus()[0].brand());
        }
        "load_avg" => {
            let load_avg = System::load_average();
            writeln!(&mut io::stdout(), "one minute     : {}%", load_avg.one);
            writeln!(&mut io::stdout(), "five minutes   : {}%", load_avg.five);
            writeln!(&mut io::stdout(), "fifteen minutes: {}%", load_avg.fifteen);
        }
        e if e.starts_with("show ") => {
            let tmp: Vec<&str> = e.split(' ').collect();

            if tmp.len() != 2 {
                writeln!(
                    &mut io::stdout(),
                    "show command takes a pid or a name in parameter!"
                );
                writeln!(&mut io::stdout(), "example: show 1254");
            } else if let Ok(pid) = Pid::from_str(tmp[1]) {
                match sys.process(pid) {
                    Some(p) => {
                        writeln!(&mut io::stdout(), "{:?}", *p);
                        writeln!(
                            &mut io::stdout(),
                            "Files open/limit: {:?}/{:?}",
                            p.open_files(),
                            p.open_files_limit(),
                        );
                    }
                    None => {
                        writeln!(&mut io::stdout(), "pid \"{pid:?}\" not found");
                    }
                }
            } else {
                let proc_name = tmp[1];
                for proc_ in sys.processes_by_name(proc_name.as_ref()) {
                    writeln!(
                        &mut io::stdout(),
                        "==== {} ====",
                        proc_.name().to_string_lossy()
                    );
                    writeln!(&mut io::stdout(), "{proc_:?}");
                }
            }
        }
        "temperature" => {
            for component in components.iter() {
                writeln!(&mut io::stdout(), "{component:?}");
            }
        }
        "network" => {
            networks::print_networks(networks);
        }
        "show" => {
            writeln!(
                &mut io::stdout(),
                "'show' command expects a pid number or a process name"
            );
        }
        e if e.starts_with("kill ") => {
            let tmp: Vec<&str> = e.split(' ').collect();

            if tmp.len() != 3 {
                writeln!(
                    &mut io::stdout(),
                    "kill command takes the pid and a signal number in parameter!"
                );
                writeln!(&mut io::stdout(), "example: kill 1254 9");
            } else {
                let pid = Pid::from_str(tmp[1]).unwrap();
                let signal = i32::from_str(tmp[2]).unwrap();

                if signal < 1 || signal > 31 {
                    writeln!(
                        &mut io::stdout(),
                        "Signal must be between 0 and 32 ! See the signals list with the \
                         signals command"
                    );
                } else {
                    match sys.process(pid) {
                        Some(p) => {
                            if let Some(res) =
                                p.kill_with(*signals.get(signal as usize - 1).unwrap())
                            {
                                writeln!(&mut io::stdout(), "kill: {res}");
                            } else {
                                writeln!(
                                    &mut io::stdout(),
                                    "kill: signal not supported on this platform"
                                );
                            }
                        }
                        None => {
                            writeln!(&mut io::stdout(), "pid not found");
                        }
                    };
                }
            }
        }
        "disks" => {
            disks::print_disks_info(disks);
        }
        "users" => {
            for user in users {
                writeln!(
                    &mut io::stdout(),
                    "{:?} => {:?}",
                    user.name(),
                    user.groups()
                );
            }
        }
        "boot_time" => {
            writeln!(&mut io::stdout(), "{} seconds", System::boot_time());
        }
        "uptime" => {
            let up = System::uptime();
            let mut uptime = up;
            let days = uptime / 86400;
            uptime -= days * 86400;
            let hours = uptime / 3600;
            uptime -= hours * 3600;
            let minutes = uptime / 60;
            writeln!(
                &mut io::stdout(),
                "{days} days {hours} hours {minutes} minutes ({up} seconds in total)",
            );
        }
        x if x.starts_with("refresh") => {
            if x == "refresh" {
                writeln!(&mut io::stdout(), "Getting processes' information...");
                sys.refresh_all();
                writeln!(&mut io::stdout(), "Done.");
            } else if x.starts_with("refresh ") {
                writeln!(&mut io::stdout(), "Getting process' information...");
                if let Some(pid) = x
                    .split(' ')
                    .filter_map(|pid| pid.parse().ok())
                    .take(1)
                    .next()
                {
                    if sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true) != 0 {
                        writeln!(&mut io::stdout(), "Process `{pid}` updated successfully");
                    } else {
                        writeln!(&mut io::stdout(), "Process `{pid}` couldn't be updated...");
                    }
                } else {
                    writeln!(&mut io::stdout(), "Invalid [pid] received...");
                }
            } else {
                writeln!(
                    &mut io::stdout(),
                    "\"{x}\": Unknown command. Enter 'help' if you want to get the commands' \
                     list.",
                );
            }
        }
        "pid" => {
            writeln!(
                &mut io::stdout(),
                "PID: {}",
                sysinfo::get_current_pid().expect("failed to get PID")
            );
        }
        "system" => {
            writeln!(
                &mut io::stdout(),
                "System name:              {}\n\
                 System kernel version:    {}\n\
                 System OS version:        {}\n\
                 System OS (long) version: {}\n\
                 System host name:         {}\n\
		 System kernel:            {}",
                System::name().unwrap_or_else(|| "<unknown>".to_owned()),
                System::kernel_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::os_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned()),
                System::host_name().unwrap_or_else(|| "<unknown>".to_owned()),
                System::kernel_long_version(),
            );
        }
        e => {
            writeln!(
                &mut io::stdout(),
                "\"{e}\": Unknown command. Enter 'help' if you want to get the commands' \
                 list.",
            );
        }
    }
    false
}

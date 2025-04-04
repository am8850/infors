//#![crate_type = "bin"]
use clap::Parser;
//use std::ffi::OsStr;

mod cmd;
mod info;

fn main() {
    let args = cmd::Cli::parse();

    match args.command {
        cmd::Commands::Memory {} => {
            let input: &str = "memory";
            info::query(input);
        }
        cmd::Commands::Cpus {} => {
            let input: &str = "cpus";
            info::query(input);
        }
        cmd::Commands::Network {} => {
            let query: &str = "network";
            info::query(query);
        }
        cmd::Commands::Temperature {} => {
            let query: &str = "temperature";
            info::query(query);
        }
        cmd::Commands::Frequency {} => {
            let query: &str = "frequency";
            info::query(query);
        }
        cmd::Commands::Disks {} => {
            let query: &str = "disks";
            info::query(query);
        }
        cmd::Commands::Users {} => {
            let query: &str = "users";
            info::query(query);
        }
        cmd::Commands::System {} => {
            let query: &str = "system";
            info::query(query);
        }
        cmd::Commands::Signals {} => {
            let query: &str = "signals";
            info::query(query);
        }
        cmd::Commands::Uptime {} => {
            let query: &str = "uptime";
            info::query(query);
        }
        cmd::Commands::LoadAvg {} => {
            let query: &str = "load_avg";
            info::query(query);
        }
        cmd::Commands::RefreshAll {} => {
            let query: &str = "refresh_all";
            info::query(query);
        }
        cmd::Commands::Boottime {} => {
            let query: &str = "boot_time";
            info::query(query);
        } // cmd::Commands::Clone { remote } => {
          //     println!("Cloning {remote}");
          //     let query: &str = "memory";
          //     info::query(query);
          // }
          // cmd::Commands::Diff {
          //     mut base,
          //     mut head,
          //     mut path,
          //     color,
          // } => {
          //     if path.is_none() {
          //         path = head;
          //         head = None;
          //         if path.is_none() {
          //             path = base;
          //             base = None;
          //         }
          //     }
          //     let base = base
          //         .as_deref()
          //         .map(|s| s.to_str().unwrap())
          //         .unwrap_or("stage");
          //     let head = head
          //         .as_deref()
          //         .map(|s| s.to_str().unwrap())
          //         .unwrap_or("worktree");
          //     let path = path.as_deref().unwrap_or_else(|| OsStr::new(""));
          //     println!(
          //         "Diffing {}..{} {} (color={})",
          //         base,
          //         head,
          //         path.to_string_lossy(),
          //         color
          //     );
          // }
          // cmd::Commands::Push { remote } => {
          //     println!("Pushing to {remote}");
          // }
          // cmd::Commands::Add { path } => {
          //     println!("Adding {path:?}");
          // }
          // cmd::Commands::Stash(stash) => {
          //     let stash_cmd = stash
          //         .command
          //         .unwrap_or(cmd::StashCommands::Push(stash.push));
          //     match stash_cmd {
          //         cmd::StashCommands::Push(push) => {
          //             println!("Pushing {push:?}");
          //         }
          //         cmd::StashCommands::Pop { stash } => {
          //             println!("Popping {stash:?}");
          //         }
          //         cmd::StashCommands::Apply { stash } => {
          //             println!("Applying {stash:?}");
          //         }
          //     }
          // }
          // cmd::Commands::External(args) => {
          //     println!("Calling out to {:?} with {:?}", &args[0], &args[1..]);
          // }
    }

    // Continued program logic goes here...
}

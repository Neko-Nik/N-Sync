mod args;
mod sync_files;

use args::Args;
use sync_files::{sync_local_files, sync_remote_files};

use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};
use std::sync::mpsc;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse_args();
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = recommended_watcher(tx)?;

    let master_dir = Path::new(&args.origin);
    watcher.watch(master_dir, RecursiveMode::Recursive)?;
    println!("Watching for changes in {}. Press Ctrl+C to stop.", master_dir.display());

    for event in rx {
        match event {
            Ok(event) => {
                println!("Detected change: {:?}", event.kind);
                match args.mode.as_str() {
                    "local" => sync_local_files(&args.origin, &args.destination),
                    "remote" => sync_remote_files(
                        &args.remote_host,
                        args.port,
                        &args.user,
                        &args.origin,
                        &args.destination,
                    ),
                    _ => std::process::exit(1),
                }
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
            }
        }
    }

    Ok(())
}

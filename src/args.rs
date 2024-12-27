use clap::Parser;
use std::path::Path;
use std::process;


#[derive(Parser, Debug)]
#[command(name = "N-sync")]
#[command(author = "Neko Nik, admin@nekonik.com")]
#[command(version = "v1.0")]
#[command(about = "N Sync - Simple Sync tool b/w servers")]
#[command(long_about = "This is a simple tool to sync files between servers")]
#[command(after_help = "Author: Neko Nik, admin@nekonik.com")]
pub struct Args {
    /// Port number to connect to the SSH
    #[arg(short, long, default_value = "22")]
    pub port: u16,

    /// User name for the SSH connection
    #[arg(short, long, default_value = "root")]
    pub user: String,

    /// Host name or IP address to connect to the SSH
    /// (Private IP or Public IP)
    #[arg(short, long, default_value = "localhost")]
    pub remote_host: String,

    /// Master or Origin path to keep the watch on and sync to the slave
    #[arg(short, long)]
    pub origin: String,

    /// Slave or Destination path to sync the files from the master to
    #[arg(short, long)]
    pub destination: String,

    /// Is 'local' or 'remote' - to sync from local to remote or remote to local
    #[arg(short, long, default_value = "local")]
    pub mode: String,
}


impl Args {
    /// Method to parse the arguments and validate them
    pub fn parse_args() -> Self {
        let args = Args::parse();

        // Validate the origin directory
        if !Path::new(&args.origin).exists() {
            eprintln!("Error: The origin directory '{}' does not exist.", args.origin);
            process::exit(1);
        }

        match args.mode.as_str() {
            "remote" => {
                // Validate remote mode-specific fields
                if args.remote_host.is_empty() || args.remote_host == "localhost" {
                    eprintln!("Error: Invalid remote host '{}'.", args.remote_host);
                    process::exit(1);
                }

                if args.user.is_empty() {
                    eprintln!("Error: Invalid user '{}'.", args.user);
                    process::exit(1);
                }

                if args.port == 0 {
                    eprintln!("Error: Invalid port number '{}'. Must be greater than 0.", args.port);
                    process::exit(1);
                }
            }
            "local" => {
                // Validate local mode-specific fields
                if args.destination.is_empty() {
                    eprintln!("Error: Destination path cannot be empty.");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("Error: Invalid mode '{}'. Mode must be either 'local' or 'remote'.", args.mode);
                process::exit(1);
            }
        }

        args
    }
}

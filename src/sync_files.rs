use std::process::Command;

pub fn sync_local_files(master_dir: &str, slave_dir: &str) {
    let rsync_command = format!(
        "rsync -az --delete {} {}",
        master_dir, slave_dir
    );

    // Run the rsync command to sync the directories
    match Command::new("sh")
        .arg("-c")
        .arg(&rsync_command)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("Sync complete.");
            } else {
                eprintln!(
                    "Error during sync: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => eprintln!("Failed to execute rsync: {}", e),
    }
}

pub fn sync_remote_files(
    remote_host: &str,
    port: u16,
    user: &str,
    master_dir: &str,
    slave_dir: &str,
) {
    let rsync_command = format!(
        "rsync -az --delete -e 'ssh -p {}' {} {}@{}:{}",
        port, master_dir, user, remote_host, slave_dir
    );

    // Run the rsync command to sync the directories with a remote host
    match Command::new("sh")
        .arg("-c")
        .arg(&rsync_command)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("Sync complete.");
            } else {
                eprintln!(
                    "Error during sync: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => eprintln!("Failed to execute rsync: {}", e),
    }
}

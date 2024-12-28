# N-Sync

N-sync is a simple and efficient file synchronization tool that allows you to sync files between local and remote servers. It leverages `rsync` for reliable file transfer and provides a file-watching capability to sync changes dynamically.

## Features

- **Local Sync**: Synchronize files between directories on the same machine.
- **Remote Sync**: Synchronize files to a remote server using SSH.
- **File Watching**: Detect changes in the source directory and sync updates dynamically.
- **Customizable Parameters**: Configure SSH user, port, and remote host with ease.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Neko-Nik/n-sync.git
   cd n-sync
   ```

2. Build the project:
   ```bash
   cargo build --release --target=x86_64-unknown-linux-gnu
   ```

3. Optionally, move the binary to a directory in your `$PATH`:
   ```bash
   mv target/x86_64-unknown-linux-gnu/release/n-sync /usr/local/bin
   ```

## Usage

### Command Line Arguments

| Argument         | Short | Default       | Description                                                 |
|------------------|-------|---------------|-------------------------------------------------------------|
| `--port`         | `-p`  | `22`          | Port number for the SSH connection.                         |
| `--user`         | `-u`  | `root`        | Username for the SSH connection.                            |
| `--remote-host`  | `-r`  | `localhost`   | Hostname or IP address of the remote server.                |
| `--origin`       | `-o`  | (Required)    | Path to the source directory to watch or sync from.         |
| `--destination`  | `-d`  | (Required)    | Path to the target directory for synchronization.           |
| `--mode`         | `-m`  | `local`       | Mode of synchronization: `local` or `remote`.               |

### Example Commands

#### Local Sync

```bash
n-sync --mode local --origin /path/to/source --destination /path/to/destination
```

#### Remote Sync

```bash
n-sync --mode remote \
       --origin /path/to/source \
       --destination /path/to/remote/destination \
       --remote-host example.com \
       --user myuser \
       --port 22
```

### Dynamic File Watching

N-sync automatically watches the `--origin` directory for changes and syncs updates to the target. To stop the process, press `Ctrl+C`.

## Dependencies

- **Rust**: Ensure you have Rust installed to build the project. [Install Rust](https://www.rust-lang.org/tools/install)
- **Rsync**: N-sync uses `rsync` for file synchronization. Ensure `rsync` is installed on both local and remote machines.
- **Tokio**: Used for asynchronous runtime.
- **Notify**: File watcher for detecting changes dynamically.

## Development

- Install dependencies:
   ```bash
   cargo build
   ```

- Test the project to ensure everything is working as expected

- Create a pull request with your changes for review


## Contributing

Contributions are welcome! Please open an issue or submit a pull request with your improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/Neko-Nik/N-Sync/blob/main/LICENSE) file for details.

## Author

Created by **Neko Nik** (<admin@nekonik.com>) - Feel free to contact me!

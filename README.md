
# Bluetooth Manager

This is a Rust program that removes and reinstalls Bluetooth packages across different Linux distributions. 
It automatically detects the package manager and performs the necessary operations to set up Bluetooth on your system.

## Features

- Detects common Linux package managers (e.g., `apt`, `dnf`, `yum`, `pacman`, `zypper`).
- Removes existing Bluetooth packages and installs new ones.
- Enables and starts the Bluetooth service.

## Installation

1. Clone this repository and navigate to the project directory.
2. Make sure you have Rust installed. If not, you can install it using [Rust's installation guide](https://www.rust-lang.org/learn/get-started).

## Usage

1. Build the executable:

   ```bash
   cargo build --release
   ```

   The executable will be located at `./target/release/bluetooth_manager`.

2. Run the program (you may need to use `sudo`):

   ```bash
   sudo ./target/release/bluetooth_manager
   ```

3. Optionally, to make the executable available globally:

   ```bash
   sudo cp ./target/release/bluetooth_manager /usr/local/bin/
   ```

   Then, you can use `bluetooth_manager` command from anywhere.

## Package Managers Supported

The program currently supports the following package managers:

- `apt` (Debian/Ubuntu-based)
- `dnf` (Fedora-based)
- `yum` (CentOS/RHEL-based)
- `pacman` (Arch-based)
- `zypper` (openSUSE-based)

## Code Overview

- `detect_package_manager`: Determines which package manager is available on the system.
- `run_command`: Executes shell commands using Rust's `Command` module.
- The main function orchestrates package removal, installation, and enabling the Bluetooth service.

## Important Notes

- The program requires elevated privileges for package installation and system modifications.
- The packages installed are `bluez`, `bluez-tools`, and `rfkill`. Adjust package names as needed.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

use std::process::Command;
use std::io;

fn main() {
    println!("Detecting package manager and setting up Bluetooth...");

    // Determine the package manager and run appropriate commands
    if let Some(package_manager) = detect_package_manager() {
        match package_manager {
            "apt" => {
                // For Debian/Ubuntu-based systems
                if run_command("sudo apt update").is_err()
                    || run_command("sudo apt remove --purge -y bluetooth bluez bluez-tools rfkill").is_err()
                    || run_command("sudo apt install -y bluetooth bluez bluez-tools rfkill").is_err()
                {
                    eprintln!("Error encountered while setting up Bluetooth with apt.");
                    return;
                }
            },
            "dnf" => {
                // For Fedora-based systems
                if run_command("sudo dnf remove -y bluez bluez-tools").is_err()
                    || run_command("sudo dnf install -y bluez bluez-tools").is_err()
                {
                    eprintln!("Error encountered while setting up Bluetooth with dnf.");
                    return;
                }
            },
            "yum" => {
                // For CentOS/RHEL-based systems
                if run_command("sudo yum remove -y bluez bluez-tools").is_err()
                    || run_command("sudo yum install -y bluez bluez-tools").is_err()
                {
                    eprintln!("Error encountered while setting up Bluetooth with yum.");
                    return;
                }
            },
            "pacman" => {
                // For Arch-based systems
                if run_command("sudo pacman -Rns --noconfirm bluez bluez-utils").is_err()
                    || run_command("sudo pacman -S --noconfirm bluez bluez-utils").is_err()
                {
                    eprintln!("Error encountered while setting up Bluetooth with pacman.");
                    return;
                }
            },
            "zypper" => {
                // For openSUSE-based systems
                if run_command("sudo zypper remove -y bluez bluez-tools").is_err()
                    || run_command("sudo zypper install -y bluez bluez-tools").is_err()
                {
                    eprintln!("Error encountered while setting up Bluetooth with zypper.");
                    return;
                }
            },
            _ => eprintln!("Unsupported package manager. Please install Bluetooth manually."),
        }

        // Enable and start the Bluetooth service
        if run_command("sudo systemctl enable bluetooth").is_err()
            || run_command("sudo systemctl start bluetooth").is_err()
        {
            eprintln!("Error enabling or starting the Bluetooth service.");
            return;
        }

        println!("Bluetooth installation and setup completed.");
    } else {
        eprintln!("No supported package manager found. Please install Bluetooth manually.");
    }
}

fn detect_package_manager() -> Option<&'static str> {
    let package_managers = [
        ("apt", "apt --version"),
        ("dnf", "dnf --version"),
        ("yum", "yum --version"),
        ("pacman", "pacman --version"),
        ("zypper", "zypper --version"),
    ];

    for (name, check_cmd) in &package_managers {
        if run_command(check_cmd).is_ok() {
            return Some(name);
        }
    }

    None
}

fn run_command(command: &str) -> io::Result<()> {
    let mut parts = command.split_whitespace();
    let cmd = parts.next().unwrap();
    let args: Vec<&str> = parts.collect();
    let status = Command::new(cmd).args(args).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command `{}` failed to execute", command),
        ))
    }
}

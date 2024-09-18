# Overview

- **privilege-rs** provides a simple and lightweight user interface for privilege escalation using the eframe GUI library. This crate is designed to facilitate requesting root privileges on Linux systems by prompting users to enter their root or admin password through a graphical interface.

# Features

- Lightweight UI for password entry using eframe.
- Automatic privilege detection.
- Secure password handling and verification.
- Re-runs the application with elevated privileges when necessary.

# Example:

- Add to your `Cargo.toml`:

```
[dependencies]
privilege-rs = "0.1.3"
```

- In your `main.rs`:

```
fn main() {
    if let Ok(privilege) = privilege_request() {
        match privilege {
            privilege_rs::Privilege::Root => {
                println!("Run the application as Root privilege");
            },
            privilege_rs::Privilege::User => {
                println!("Failed to request privilege");
            },
            privilege_rs::Privilege::Suid => {
                println!("Run the application as Root privilege");
            },
        }
    }
}
```

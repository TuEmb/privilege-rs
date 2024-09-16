use eframe::egui;
use std::process::{Command, Stdio};

#[derive(PartialEq)]
enum Privilege {
    Root,
    User,
    Suid,
}

/// Check current privilege:
///  - If Root/Suid privilege -> donothing
///  - If User privilege -> password request and re-run the application.
pub fn privilege_request() {
    if get_privilege() == Privilege::User {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]), // Adjusted size
            ..Default::default()
        };

        let mut password = String::new();
        let mut notification = String::new();

        let _ = eframe::run_simple_native("Privilege Request", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // Center the content
                ui.vertical_centered(|ui| {
                    ui.heading("Privilege Request");

                    ui.add_space(20.0); // Add space for better layout

                    // Password input section
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Enter admin/root password: ");
                        ui.add_space(5.0); // Add space between label and input
                        ui.add(egui::TextEdit::singleline(&mut password).password(true))
                            .labelled_by(name_label.id);
                    });

                    ui.add_space(10.0); // Add space before the button

                    // Submit button
                    if ui.button("Submit").clicked()
                        || ui.input(|i| i.key_pressed(egui::Key::Enter))
                    {
                        if verify_password(&password) {
                            let _ = run_with_privilege(password.clone());
                        } else {
                            notification = "Failed to verify the password.".to_owned();
                        }
                    }

                    // Notification display
                    if !notification.is_empty() {
                        ui.add_space(15.0); // Add space before notification
                        ui.label(egui::RichText::new(&notification).color(egui::Color32::RED));
                    }
                });
            });
        });
    }
}

/// Use getuid() and geteuid() to know current privilege
fn get_privilege() -> Privilege {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };

    match (uid, euid) {
        (0, 0) => Privilege::Root,
        (_, 0) => Privilege::Suid,
        (_, _) => Privilege::User,
    }
}

/// Run a dummy command with the password first to verify
fn verify_password(password: &str) -> bool {
    let mut command = Command::new("sudo");
    let mut child = command
        .arg("-k") // Invalidate cached credentials
        .arg("-S") // Read password from stdin
        .arg("true") // Dummy command to verify password
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to execute sudo");

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        writeln!(stdin, "{}", password).expect("Failed to write to stdin");
    }

    let ecode = child.wait().expect("Failed to wait on child");

    ecode.success()
}

/// Re-run the application with sudo password
fn run_with_privilege(pwd: String) -> std::io::Result<Privilege> {
    let current = get_privilege();
    match current {
        Privilege::Root => {
            return Ok(current);
        }
        Privilege::Suid => {
            unsafe {
                libc::setuid(0);
            }
            return Ok(current);
        }
        Privilege::User => {
            println!("Escalating privileges");
        }
    }
    let mut args: Vec<_> = std::env::args().collect();
    if let Some(absolute_path) = std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|p| p.to_string()))
    {
        args[0] = absolute_path;
    }

    let mut command: Command = Command::new("sudo");
    let mut child = command
        .arg("-S")
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        writeln!(stdin, "{}", pwd).expect("Failed to write to stdin");
        // Close stdin to signal no more input will be provided
    }
    std::process::exit(0);
}

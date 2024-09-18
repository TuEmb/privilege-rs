use privilege_rs::privilege_request;

fn main() {
    if let Ok(privilege) = privilege_request() {
        match privilege {
            privilege_rs::Privilege::Root => {
                println!("Run the application as Root privilege");
            }
            privilege_rs::Privilege::User => {
                println!("Failed to request privilege");
            }
            privilege_rs::Privilege::Suid => {
                println!("Run the application as Root privilege");
            }
        }
    }
}

use std::fs;
use std::io::{stdin, stdout, BufRead, Write};

fn main() {
    println!("Welcome to the setup for 'NebularStore'");
    let install_path = user_input("Path for installation: ");
    let company_name = user_input("Company name: ");
    let backend_port = user_input("Backend port: ");
    let frontend_port = user_input("Frontend port: ");
    let admin_password = user_input("Admin password: ");
    let mut reveal_md_yn = false;
    let mut reveal_md_port = None;
    if user_input("Do you want to add the 'reveal_md' plugin? (y/n): ").to_lowercase() == "y" {
        reveal_md_yn = true;
        reveal_md_port = Some(user_input("Reveal md port: "));
    }

    let compose = include_str!("../resources/templates/compose_template.yml");
    let general = include_str!("../resources/templates/general_template.toml");
    let admin = include_str!("../resources/templates/admin_template.toml");
    let client = include_str!("../resources/templates/client_example.env");
    let icon = include_bytes!("../resources/templates/icon_template.jpg");
    let reveal_md = include_str!("../resources/templates/reveal_md_template.yml");

    println!("Generating directories");
    fs::create_dir_all(format!("{}/data/config", &install_path))
        .expect("Failed to create data directories");
    fs::create_dir_all(format!("{}/data/repository", &install_path))
        .expect("Failed to create data directories");

    let install_path_buf = fs::canonicalize(install_path).unwrap();
    let install_path = install_path_buf
        .to_str()
        .unwrap()
        .trim_start_matches("\\\\?\\");

    println!("Generating files");
    let compose = compose
        .replace("$1", &frontend_port)
        .replace("$2", &backend_port)
        .replace("$3", &format!("{}/data", &install_path))
        .replace(
            "$4",
            &if reveal_md_yn {
                reveal_md
                    .replace("$1", &reveal_md_port.clone().unwrap_or("1849".to_string()))
                    .replace("$2", &format!("{}/data/repository", &install_path))
                    .to_string()
            } else {
                String::new()
            },
        );
    let general = general.replace("$1", &company_name);
    let admin = admin.replace("$1", &admin_password);
    let client = client
        .replace("$1", &backend_port)
        .replace("$2", &reveal_md_port.unwrap_or("1948".to_string()))
        .replace("$3", &reveal_md_yn.to_string());

    println!("Writing files");
    fs::write(
        format!("{}/data/config/general.toml", &install_path),
        general,
    )
    .expect("Failed to write admin config");
    fs::write(format!("{}/data/config/admin.toml", &install_path), admin)
        .expect("Failed to write admin config");
    fs::write(format!("{}/data/config/icon.jpg", &install_path), icon)
        .expect("Failed to write admin config");
    fs::write(format!("{}/client.env", &install_path), client)
        .expect("Failed to write admin config");
    fs::write(format!("{}/compose.yml", &install_path), compose)
        .expect("Failed to write admin config");

    println!("Finished setup");
    println!("For configuration look into the /data/config/ folder");
    println!("To run the application run 'docker compose up'")
}

fn user_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().lock().flush().unwrap();

    let mut content = String::new();
    stdin().lock().read_line(&mut content).unwrap();

    content.trim().to_string()
}

mod models;
mod services;

use crate::services::openpaye_client::OpenpayeClient;
use services::openpaye_client::ApiCredentials;
use std::io::{stdout, Write};

#[tokio::main]
async fn main() {
    println!("=== OpenPaye CLI ===");

    let client = init_client();

    print!("Code du dossier (laisser vide pour tout récupérer): ");
    stdout().flush().unwrap();
    let mut folder_code = String::new();
    std::io::stdin().read_line(&mut folder_code).unwrap();
    folder_code = folder_code.trim().to_string();

    let result = client.fetch_folders(if folder_code.is_empty() { None } else { Some(folder_code) }).await;

    match result {
        Ok(folders) => {
            println!("=== {} dossier(s) trouvé(s) ===\n\n", folders.len());
            for (i, folder) in folders.iter().enumerate() {
                println!("- {} {}\n", i + 1, folder);
            }
        }
        Err(e) => {
            eprintln!("Erreur: {}", e);
        }
    }

    println!("\nPress any key to exit...");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

fn init_client() -> OpenpayeClient {
    print!("Email: ");
    stdout().flush().unwrap();
    let mut email = String::new();
    std::io::stdin().read_line(&mut email).unwrap();

    print!("Mot de passe: ");
    stdout().flush().unwrap();
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();

    let credentials = ApiCredentials::new(
        email.trim(),
        password.trim(),
    );

    OpenpayeClient::new(credentials)
}

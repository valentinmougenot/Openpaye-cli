use std::io::{stdout, Write};
use serde::{Deserialize, Serialize};
use reqwest::Client;

struct ApiCredentials {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct FolderResponse {
    total_count: u32,
    total_pages: u32,
    dossiers: Vec<Folder>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Folder {
    id: u32,
    code: String,
    siret: String,
    nom_dossier: String,
    adresse_email: Option<String>,
    telephone: Option<String>,
    nom_contact: Option<String>,
    qualite: Option<String>,
    annee: Option<String>
}

const API_BASE_URL: &str = "https://api.openpaye.co";

async fn fetch_data(credentials: &ApiCredentials, folder_code: Option<String>) -> Result<Vec<Folder>, String> {

    let client = Client::new();
    let mut result = Vec::new();
    let mut page = 0;

    loop {
        let url = format!("{}/dossiers?page={}", API_BASE_URL, page);

        let response = client
            .get(&url)
            .basic_auth(&credentials.email, Some(&credentials.password))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "API request failed with status: {}",
                response.status()
            ));
        }

        let data: FolderResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        result.extend(data.dossiers);

        page += 1;

        if page >= data.total_pages {
            break;
        }
    }

    if let Some(folder_code) = folder_code {
        result.retain(|folder| folder.code == folder_code);
    }

    Ok(result)
}

#[tokio::main]
async fn main() {
    println!("=== OpenPaye CLI ===");
    print!("Email: ");
    stdout().flush().unwrap();
    let mut email = String::new();
    std::io::stdin().read_line(&mut email).unwrap();

    print!("Mot de passe: ");
    stdout().flush().unwrap();
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();

    let credentials = ApiCredentials {
        email: email.trim().to_string(),
        password: password.trim().to_string(),
    };

    print!("Code du dossier (laisser vide pour tout récupérer): ");
    stdout().flush().unwrap();
    let mut folder_code = String::new();
    std::io::stdin().read_line(&mut folder_code).unwrap();
    folder_code = folder_code.trim().to_string();

    let result = fetch_data(&credentials, if folder_code.is_empty() { None } else { Some(folder_code) }).await;

    match result {
        Ok(folders) => {
            println!("=== {} dossier(s) trouvé(s) ===", folders.len());
            for folder in folders {
                println!("- {:?}", folder);
            }
        }
        Err(e) => {
            eprintln!("Erreur: {}", e);
        }
    }
}

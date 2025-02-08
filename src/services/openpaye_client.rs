use crate::models::folder::{Folder, FolderResponse};
use futures::future::join_all;
use reqwest::Client;

const API_BASE_URL: &str = "https://api.openpaye.co";

pub struct ApiCredentials {
    email: String,
    password: String,
}

impl ApiCredentials {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

pub struct OpenpayeClient {
    credentials: ApiCredentials,
}

impl OpenpayeClient {
    pub fn new(credentials: ApiCredentials) -> Self {
        Self { credentials }
    }

    pub async fn fetch_folders(&self, folder_code: Option<String>) -> Result<Vec<Folder>, String> {
        let client = Client::new();

        let url = format!("{}/dossiers?page=0", API_BASE_URL);
        let response = client
            .get(&url)
            .basic_auth(&self.credentials.email, Some(&self.credentials.password))
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API request failed with status: {}", response.status()));
        }

        let first_page: FolderResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let total_pages = first_page.total_pages;
        let mut result = first_page.folders;

        if total_pages > 1 {
            let futures_vec = (1..total_pages).map(|page| {
                let client = client.clone();
                let url = format!("{}/dossiers?page={}", API_BASE_URL, page);
                async move {
                    let email = self.credentials.email.clone();
                    let password = self.credentials.password.clone();
                    let resp = client
                        .get(&url)
                        .basic_auth(&email, Some(&password))
                        .send()
                        .await
                        .map_err(|e| format!("Failed to send request for page {}: {}", page, e))?;

                    if !resp.status().is_success() {
                        return Err(format!("API request for page {} failed with status: {}", page, resp.status()));
                    }

                    let data: FolderResponse = resp
                        .json()
                        .await
                        .map_err(|e| format!("Failed to parse JSON for page {}: {}", page, e))?;
                    Ok(data.folders)
                }
            });

            let pages: Vec<Result<Vec<Folder>, String>> = join_all(futures_vec).await;
            for page in pages {
                result.extend(page?);
            }
        }

        if let Some(ref code) = folder_code {
            result.retain(|folder| folder.code.as_ref().map_or(false, |c| c == code));
        }

        Ok(result)
    }
}
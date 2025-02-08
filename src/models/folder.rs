use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct FolderResponse {
    pub total_count: u32,
    pub total_pages: u32,
    #[serde(rename = "dossiers")]
    pub folders: Vec<Folder>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    pub id: u32,
    pub code: Option<String>,
    pub siret: Option<String>,
    #[serde(rename = "nom_dossier")]
    pub name: String,
    #[serde(rename = "adresse_email")]
    pub email: Option<String>,
    #[serde(rename = "telephone")]
    pub telephone: Option<String>,
    #[serde(rename = "nom_contact")]
    pub contact: Option<String>,
    #[serde(rename = "qualite")]
    pub quality: Option<u32>,
    #[serde(rename = "annee")]
    pub year: Option<String>,
}

impl Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ id: {}, code: {}, nom_dossier: {}, siret: {} }}",
               self.id,
               self.code.as_deref().unwrap_or("null"),
               self.name,
               self.siret.as_deref().unwrap_or("null"))
    }
}
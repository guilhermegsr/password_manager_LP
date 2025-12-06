use serde::{Serialize, Deserialize};
use base64::{engine::general_purpose, Engine as _};
use dotenvy;

use password_manager::{
    infrastructure::{database::get_database_connection, logger::init_logger},
    services::{
        auth_service::{AuthService, Session},
        credential_service::CredentialService,
    },
    models::{
        credential::Credential,
        user::User,
    },
};

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionDTO {
    pub username: String,
    pub vault_id: String,
    pub vault_key: String,
    pub passphrase: String,
}

impl SessionDTO {
    pub fn into_session(self) -> Result<Session, String> {
        let vault_key_bytes = general_purpose::STANDARD
            .decode(&self.vault_key)
            .map_err(|e| format!("Erro ao decodificar vault_key: {}", e))?;

        Ok(Session {
            user: User::from_session(self.username.clone()),
            vault_id: Uuid::parse_str(&self.vault_id)
                .map_err(|e| format!("vault_id inválido: {}", e))?,
            vault_key: vault_key_bytes,
            passphrase: self.passphrase,
        })
    }
}

fn load_env() {
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
    let env_file = format!(".env.{}", app_env);
    let _ = dotenvy::from_filename(env_file);
}

#[tauri::command]
fn register_user(username: String, password: String) -> Result<(), String> {
    AuthService::register(&username, &password).map_err(|e| e.to_string())
}

#[tauri::command]
fn login_user(username: String, password: String) -> Result<SessionDTO, String> {
    let session = AuthService::login(&username, &password)
        .map_err(|e| e.to_string())?;

    Ok(SessionDTO {
        username: session.user.username().to_string(),
        vault_id: session.vault_id.to_string(),
        vault_key: general_purpose::STANDARD.encode(&session.vault_key),
        passphrase: session.passphrase.clone(),
    })
}

#[tauri::command]
fn create_credential(
    session: SessionDTO,
    name: String,
    username: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    password: Option<String>,
) -> Result<(), String> {
    let session = session.into_session()?;
    let notes_bytes = notes.map(|n| n.into_bytes());

    CredentialService::create(
        &session,
        &name,
        username,
        url,
        notes_bytes,
        password.as_deref(),
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn list_credentials(session: SessionDTO) -> Result<Vec<Credential>, String> {
    let session = session.into_session()?;
    CredentialService::list(&session).map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct CredentialFullDTO {
    password: Option<String>,
    notes: Option<String>,
}

#[tauri::command]
fn get_credential_full(session: SessionDTO, id: String) -> Result<CredentialFullDTO, String> {
    let session = session.into_session()?;
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let password = CredentialService::reveal_password(&session, uuid)
        .map_err(|e| e.to_string())?;

    let notes = CredentialService::reveal_notes(&session, uuid)
        .map_err(|e| e.to_string())?;

    Ok(CredentialFullDTO { password, notes })
}

#[tauri::command]
fn update_credential(
    session: SessionDTO,
    id: String,
    name: Option<String>,
    username: Option<String>,
    url: Option<String>,
    notes: Option<String>,
    password: Option<String>,
) -> Result<(), String> {
    let session = session.into_session()?;
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let credential = CredentialService::get(&session, uuid)
        .map_err(|e| e.to_string())?;

    let notes_bytes = notes.map(|n| n.into_bytes());

    CredentialService::update(
        &session,
        credential,
        name,
        username,
        url,
        notes_bytes,
        password.as_deref(),
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_credential(session: SessionDTO, id: String) -> Result<(), String> {
    let session = session.into_session()?;
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    CredentialService::delete(&session, uuid).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    load_env();
    init_logger();
    get_database_connection().expect("Falha ao inicializar banco de dados");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            register_user,
            login_user,
            create_credential,
            list_credentials,
            get_credential_full,
            update_credential,
            delete_credential
        ])
        .run(tauri::generate_context!())
        .expect("erro ao executar aplicação Tauri");
}

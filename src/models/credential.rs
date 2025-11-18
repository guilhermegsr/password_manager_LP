use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entidade de domínio que representa uma credencial armazenada em um cofre.
///
/// Campos como nome de usuário, URL, notas e senha são opcionais.
/// Tanto a senha quanto as notas, quando presentes, são sempre armazenadas de forma cifrada
/// para evitar exposição de informações sensíveis mesmo em caso de acesso indevido ao banco de dados.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credential {
    id: Uuid,
    vault_id: Uuid,
    name: String,
    username: Option<String>,
    url: Option<String>,
    notes: Option<Vec<u8>>,
    password_cipher: Option<Vec<u8>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Credential {
    /// Cria uma nova credencial validando dados obrigatórios.
    ///
    /// ### Parâmetros
    /// - `vault_id`: Identificador do cofre ao qual a credencial pertence.
    /// - `name`: Nome da credencial (ex.: "GitHub", "Banco XYZ").
    /// - `username`: Nome de usuário associado (opcional).
    /// - `url`: Endereço da aplicação/serviço (opcional).
    /// - `notes`: Notas criptografadas (opcional).
    /// - `password_cipher`: Senha cifrada (opcional).
    ///
    /// ### Retorno
    /// - `Ok(Credential)` quando válida.
    /// - `Err(anyhow)` quando `vault_id` inválido ou `name` em branco.
    pub fn new(
        vault_id: Uuid,
        name: String,
        username: Option<String>,
        url: Option<String>,
        notes: Option<Vec<u8>>,
        password_cipher: Option<Vec<u8>>,
    ) -> Result<Self> {
        validate_uuid(vault_id)?;
        validate_name(&name)?;

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            vault_id,
            name,
            username,
            url,
            notes,
            password_cipher,
            created_at: now,
            updated_at: now,
        })
    }

    // Getters

    /// Retorna o ID único da credencial.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Retorna o ID do cofre ao qual a credencial pertence.
    pub fn vault_id(&self) -> Uuid {
        self.vault_id
    }

    /// Retorna o nome da credencial.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retorna o nome de usuário associado, quando existir.
    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    /// Retorna a URL associada à credencial, quando existir.
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    /// Retorna notas criptografadas, quando existir.
    pub fn notes(&self) -> Option<&[u8]> {
        self.notes.as_deref()
    }

    /// Retorna a senha cifrada, quando existir.
    pub fn password_cipher(&self) -> Option<&[u8]> {
        self.password_cipher.as_deref()
    }

    /// Timestamp de criação.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Timestamp da última atualização.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // Setters

    /// Atualiza o nome da credencial.
    pub fn set_name(&mut self, name: String) -> Result<()> {
        validate_name(&name)?;
        self.name = name;
        self.touch();
        Ok(())
    }

    /// Atualiza o nome de usuário.
    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
        self.touch();
    }

    /// Atualiza a URL associada.
    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url;
        self.touch();
    }

    /// Atualiza as notas criptografadas.
    pub fn set_notes(&mut self, notes: Option<Vec<u8>>) {
        self.notes = notes;
        self.touch();
    }

    /// Atualiza a senha cifrada.
    pub fn set_password_cipher(&mut self, cipher: Option<Vec<u8>>) {
        self.password_cipher = cipher;
        self.touch();
    }

    /// Atualiza o timestamp de modificação.
    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Reidrata uma credencial já persistida.
    pub(crate) fn from_persisted(
        id: Uuid,
        vault_id: Uuid,
        name: String,
        username: Option<String>,
        url: Option<String>,
        notes: Option<Vec<u8>>,
        password_cipher: Option<Vec<u8>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            vault_id,
            name,
            username,
            url,
            notes,
            password_cipher,
            created_at,
            updated_at,
        }
    }
}

// -------------------------
// Validações de domínio
// -------------------------

/// Valida se o UUID não é nulo.
fn validate_uuid(id: Uuid) -> Result<()> {
    if id.is_nil() {
        return Err(anyhow!("O ID do cofre não pode ser nulo."));
    }
    Ok(())
}

/// Valida nome da credencial.
fn validate_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(anyhow!("O nome da credencial não pode ser vazio."));
    }
    if name.len() > 64 {
        return Err(anyhow!(
            "O nome da credencial excede o limite de 64 caracteres."
        ));
    }
    Ok(())
}

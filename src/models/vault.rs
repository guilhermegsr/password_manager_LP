use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entidade de domínio que representa um cofre criptográfico.
///
/// O cofre armazena dados sigilosos pertencentes a um usuário e é protegido
/// por uma chave criptografada (vault_key_cipher). Esta chave somente deve
/// ser descriptografada no momento de uso dentro da camada de serviço.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vault {
    id: Uuid,
    user_id: Uuid,
    vault_key_cipher: Vec<u8>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Vault {
    /// Cria um novo cofre aplicando validações essenciais.
    ///
    /// ### Parâmetros
    /// - `user_id`: Identificador do usuário proprietário do cofre.
    /// - `vault_key_cipher`: Chave do cofre criptografada (não deve ser exposta).
    ///
    /// ### Retorno
    /// - `Ok(Vault)`: Instância pronta para persistência.
    /// - `Err(anyhow)`: Quando o `user_id` é inválido ou a chave está vazia.
    ///
    /// ### Aplicação
    /// Utilizado no fluxo de criação inicial do cofre após cadastro do usuário.
    /// Normalmente consumido por `VaultService` e persistido por `VaultRepository`.
    pub fn new(user_id: Uuid, vault_key_cipher: Vec<u8>) -> Result<Self> {
        validate_uuid(user_id)?;
        validate_vault_key(&vault_key_cipher)?;

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            vault_key_cipher,
            created_at: now,
            updated_at: now,
        })
    }

    /// Retorna o identificador único do cofre.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Retorna o ID do usuário proprietário.
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    /// Retorna o cipher contendo a chave criptografada do cofre.
    pub fn vault_key_cipher(&self) -> &[u8] {
        &self.vault_key_cipher
    }

    /// Retorna a data de criação do registro.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Retorna a data da última atualização.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Reidrata um cofre já persistido sem validações de domínio.
    ///
    /// ### Parâmetros
    /// - `id`: Identificador único do cofre.
    /// - `user_id`: Identificador do usuário associado.
    /// - `vault_key_cipher`: Chave criptografada armazenada.
    /// - `created_at`: Timestamp de criação persistido.
    /// - `updated_at`: Timestamp da última atualização persistido.
    ///
    /// ### Retorno
    /// - `Vault` restaurado a partir do armazenamento local.
    ///
    /// ### Aplicação
    /// Usado exclusivamente pelo repositório na leitura de registros.
    pub(crate) fn from_persisted(
        id: Uuid,
        user_id: Uuid,
        vault_key_cipher: Vec<u8>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            vault_key_cipher,
            created_at,
            updated_at,
        }
    }
}

/// Valida se o UUID do usuário é válido e não é nulo.
///
/// ### Parâmetros
/// - `id`: UUID a validar.
///
/// ### Retorno
/// - `Ok(())`: Quando válido.
/// - `Err(anyhow)`: Quando inválido.
fn validate_uuid(id: Uuid) -> Result<()> {
    if id.is_nil() {
        return Err(anyhow!("O ID do usuário não pode ser nulo."));
    }
    Ok(())
}

/// Valida a integridade da chave criptografada do cofre.
///
/// ### Parâmetros
/// - `vault_key_cipher`: Referência ao vetor de bytes contendo a chave criptografada.
///
/// ### Retorno
/// - `Ok(())`: Quando possui conteúdo.
/// - `Err(anyhow)`: Quando está vazia.
fn validate_vault_key(vault_key_cipher: &[u8]) -> Result<()> {
    if vault_key_cipher.is_empty() {
        return Err(anyhow!(
            "A chave criptografada do cofre não pode ser vazia."
        ));
    }
    Ok(())
}

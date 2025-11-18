use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entidade de domínio que representa um usuário cadastrado no sistema.
///
/// Esta entidade é utilizada pela camada de domínio e serviços de autenticação.
/// Armazena somente dados essenciais, incluindo o hash da senha (nunca a senha original).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: Uuid,
    username: String,
    password_hash: Vec<u8>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// Cria um novo usuário aplicando regras mínimas de validação de domínio.
    ///
    /// ### Parâmetros
    /// - `username`: Nome de identificação pública do usuário.
    /// - `password_hash`: Hash criptográfico resultante do processo de derivação de senha.
    ///
    /// ### Retorno
    /// - `Ok(User)`: Instância válida pronta para persistência.
    /// - `Err(anyhow)`: Caso as regras de domínio sejam violadas.
    ///
    /// ### Aplicação
    /// Utilizado no fluxo de **cadastro** ou **criação interna automática** de contas.
    /// Normalmente consumido por `UserService` e posteriormente persistido pelo `UserRepository`.
    pub fn new(username: String, password_hash: Vec<u8>) -> Result<Self> {
        validate_username(&username)?;
        validate_password_hash(&password_hash)?;

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            username,
            password_hash,
            created_at: now,
            updated_at: now,
        })
    }

    /// Obtém o identificador único do usuário.
    ///
    /// ### Retorno
    /// - `Uuid` representando o ID persistente do registro.
    ///
    /// ### Aplicação
    /// Utilizado para associações em tabelas relacionadas ou validações internas.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Obtém o nome de usuário.
    ///
    /// ### Retorno
    /// - `&str` contendo o identificador público do usuário.
    ///
    /// ### Aplicação
    /// Utilizado em autenticação, auditoria e exibição controlada ao cliente.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Obtém o hash de senha.
    ///
    /// ### Retorno
    /// - `&[u8]` com os bytes do hash criptográfico.
    ///
    /// ### Aplicação
    /// Utilizado exclusivamente em processos de verificação de senha.
    pub fn password_hash(&self) -> &[u8] {
        &self.password_hash
    }

    /// Obtém a data de criação do registro.
    ///
    /// ### Retorno
    /// - `DateTime<Utc>` com timestamp da criação.
    ///
    /// ### Aplicação
    /// Usado em auditorias, logs e exibição administrativa.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Obtém a data da última atualização do registro.
    ///
    /// ### Retorno
    /// - `DateTime<Utc>` com timestamp da última modificação.
    ///
    /// ### Aplicação
    /// Usado em fluxos de gerenciamento de senhas, auditoria e sincronização.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Reidrata um `User` a partir de dados já persistidos (sem validação de domínio).
    ///
    /// ### Parâmetros
    /// - `id`: Identificador único previamente armazenado.
    /// - `username`: Nome de usuário conforme registro existente.
    /// - `password_hash`: Hash criptográfico já persistido.
    /// - `created_at`: Timestamp original de criação.
    /// - `updated_at`: Timestamp da última atualização.
    ///
    /// ### Retorno
    /// - `User` restaurado, pronto para uso interno.
    ///
    /// ### Aplicação
    /// Usado por repositórios ao reconstruir a entidade a partir da base
    /// de dados, evitando aplicar novamente validações de criação.
    pub(crate) fn from_persisted(
        id: Uuid,
        username: String,
        password_hash: Vec<u8>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            username,
            password_hash,
            created_at,
            updated_at,
        }
    }
}

/// Valida regras comerciais de nome de usuário.
///
/// ### Parâmetros
/// - `username`: valor informado para criação do usuário.
///
/// ### Retorno
/// - `Ok(())`: quando válido.
/// - `Err(anyhow)`: quando viola requisitos mínimos.
///
/// ### Aplicação
/// Aplicado no fluxo de criação, garantindo consistência do domínio.
fn validate_username(username: &str) -> Result<()> {
    if username.trim().is_empty() {
        return Err(anyhow!("O nome de usuário não pode ser vazio."));
    }
    if username.len() < 3 || username.len() > 32 {
        return Err(anyhow!(
            "O nome de usuário deve ter entre 3 e 32 caracteres."
        ));
    }

    let re = Regex::new(r"^[a-zA-Z0-9_.-]+$").unwrap();
    if !re.is_match(username) {
        return Err(anyhow!("O nome de usuário contém caracteres inválidos."));
    }

    Ok(())
}

/// Valida integridade mínima do hash criptográfico.
///
/// ### Parâmetros
/// - `password_hash`: bytes do hash gerado após derivação.
///
/// ### Retorno
/// - `Ok(())`: quando os dados estão íntegros.
/// - `Err(anyhow)`: quando o valor está vazio.
///
/// ### Aplicação
/// Utilizado diretamente antes da criação do usuário
/// para evitar estados inválidos de segurança.
fn validate_password_hash(password_hash: &[u8]) -> Result<()> {
    if password_hash.is_empty() {
        return Err(anyhow!("O hash de senha não pode ser vazio."));
    }
    Ok(())
}

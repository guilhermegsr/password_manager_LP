use anyhow::{Result, anyhow};
use tracing::info;
use uuid::Uuid;
use zeroize::Zeroize;

use crate::{
    infrastructure::crypto::{decrypt_with_passphrase, encrypt_with_passphrase},
    models::credential::Credential,
    repositories::credential_repository::CredentialRepository,
    services::auth_service::Session,
};

/// Serviço responsável pelas operações de criação, atualização, consulta,
/// remoção e exposição controlada de credenciais protegidas em um cofre.
pub struct CredentialService;

impl CredentialService {
    /// Registra uma nova credencial no cofre do usuário autenticado.
    ///
    /// ### Parâmetros
    /// - `session`: Sessão autenticada que contém o `vault_id` e a passphrase.
    /// - `name`: Nome da credencial (ex.: "GitHub").
    /// - `username`: Nome de usuário associado (opcional).
    /// - `url`: URL de acesso à aplicação ou serviço (opcional).
    /// - `notes`: Notas criptografadas relacionadas à credencial (opcional).
    /// - `password`: Senha em texto plano que será criptografada (opcional).
    ///
    /// ### Retorno
    /// - `Ok(Credential)`: credencial criada e persistida com sucesso.
    /// - `Err(anyhow)`: falha de validação, criptografia ou persistência.
    ///
    /// ### Aplicação
    /// Utilizado durante o processo de inclusão de uma credencial pelo usuário.
    pub fn create(
        session: &Session,
        name: &str,
        username: Option<String>,
        url: Option<String>,
        notes: Option<Vec<u8>>,
        password: Option<&str>,
    ) -> Result<Credential> {
        info!(
            "Criando credencial name='{}' para vault_id='{}'",
            name, session.vault_id
        );

        let cipher = if let Some(pwd) = password {
            Some(encrypt_with_passphrase(
                &session.passphrase,
                pwd.as_bytes(),
            )?)
        } else {
            None
        };

        let notes_cipher = if let Some(n) = notes {
            Some(encrypt_with_passphrase(&session.passphrase, &n)?)
        } else {
            None
        };

        let credential = Credential::new(
            session.vault_id,
            name.to_string(),
            username,
            url,
            notes_cipher,
            cipher,
        )?;

        CredentialRepository::create(&credential)?;
        info!(
            "Credencial criada com sucesso id='{}' name='{}'",
            credential.id(),
            credential.name()
        );
        Ok(credential)
    }

    /// Atualiza uma credencial existente pertencente ao cofre do usuário autenticado.
    ///
    /// ### Parâmetros
    /// - `session`: Sessão autenticada.
    /// - `credential`: Instância já carregada da credencial a ser atualizada.
    /// - `name`: Novo nome da credencial (opcional).
    /// - `username`: Novo nome de usuário associado (opcional).
    /// - `url`: Nova URL de acesso (opcional).
    /// - `notes`: Novas notas criptografadas (opcional).
    /// - `password`: Nova senha em texto plano para criptografia (opcional).
    ///
    /// ### Retorno
    /// - `Ok(())`: atualização realizada com sucesso.
    /// - `Err(anyhow)`: credencial pertencente a outro cofre ou falha de persistência.
    ///
    /// ### Aplicação
    /// Utilizado quando o usuário edita qualquer informação da credencial.
    pub fn update(
        session: &Session,
        mut credential: Credential,
        name: Option<String>,
        username: Option<String>,
        url: Option<String>,
        notes: Option<Vec<u8>>,
        password: Option<&str>,
    ) -> Result<()> {
        info!(
            "Atualizando credencial id='{}' name='{}'",
            credential.id(),
            credential.name()
        );

        if credential.vault_id() != session.vault_id {
            return Err(anyhow!("Registro não disponível no momento"));
        }

        if let Some(value) = name {
            credential.set_name(value)?;
        }
        if let Some(value) = username {
            credential.set_username(Some(value));
        }
        if let Some(value) = url {
            credential.set_url(Some(value));
        }
        if let Some(value) = notes {
            credential.set_notes(Some(encrypt_with_passphrase(&session.passphrase, &value)?));
        }
        if let Some(pwd) = password {
            credential.set_password_cipher(Some(encrypt_with_passphrase(
                &session.passphrase,
                pwd.as_bytes(),
            )?));
        }

        CredentialRepository::update(&credential)?;
        info!("Credencial atualizada com sucesso id='{}'", credential.id());
        Ok(())
    }

    /// Remove uma credencial do cofre.
    ///
    /// ### Parâmetros
    /// - `id`: Identificador único da credencial.
    ///
    /// ### Retorno
    /// - `Ok(())`: registro removido (ou inexistente sem erro).
    /// - `Err(anyhow)`: falha no processo de remoção.
    pub fn delete(session: &Session, id: Uuid) -> Result<()> {
        info!("Solicitação de remoção credencial id='{}'", id);

        if let Some(cred) = CredentialRepository::find_by_id(id)? {
            if cred.vault_id() != session.vault_id {
                return Err(anyhow!("Registro não disponível no momento"));
            }

            CredentialRepository::delete(id)?;
            info!("Credencial removida com sucesso id='{}'", id);
            return Ok(());
        }

        Ok(())
    }

    /// Lista todas as credenciais pertencentes ao cofre do usuário autenticado.
    ///
    /// ### Retorno
    /// - `Ok(Vec<Credential>)`: podendo retornar lista vazia.
    /// - `Err(anyhow)`: falha durante a consulta.
    pub fn list(session: &Session) -> Result<Vec<Credential>> {
        info!("Listando credenciais para vault_id='{}'", session.vault_id);
        Ok(CredentialRepository::find_all_by_vault_id(
            session.vault_id,
        )?)
    }

    /// Recupera uma credencial específica, garantindo propriedade do cofre.
    ///
    /// ### Parâmetros
    /// - `cred_id`: Identificador da credencial consultada.
    ///
    /// ### Retorno
    /// - `Ok(Credential)` quando localizada.
    /// - `Err(anyhow)` quando não existir ou não pertencer ao usuário logado.
    pub fn get(session: &Session, cred_id: Uuid) -> Result<Credential> {
        info!("Consultando dados da credencial id='{}'", cred_id);

        let cred = CredentialRepository::find_by_id(cred_id)?
            .ok_or_else(|| anyhow!("Registro não disponível no momento"))?;

        if cred.vault_id() != session.vault_id {
            return Err(anyhow!("Registro não disponível no momento"));
        }

        Ok(cred)
    }

    /// Retorna a senha descriptografada de uma credencial, quando existir.
    ///
    /// ### Parâmetros
    /// - `cred_id`: Identificador da credencial.
    ///
    /// ### Retorno
    /// - `Ok(Some(String))`: senha revelada.
    /// - `Ok(None)`: credencial sem senha armazenada.
    /// - `Err(anyhow)`: falha ao consultar ou descriptografar.
    pub fn reveal_password(session: &Session, cred_id: Uuid) -> Result<Option<String>> {
        info!(
            "Solicitação de exibição de senha credencial id='{}'",
            cred_id
        );

        let cred = CredentialRepository::find_by_id(cred_id)?
            .ok_or_else(|| anyhow!("Registro não disponível no momento"))?;

        if cred.vault_id() != session.vault_id {
            return Err(anyhow!("Registro não disponível no momento"));
        }

        if let Some(cipher) = cred.password_cipher() {
            let mut plain = decrypt_with_passphrase(&session.passphrase, cipher)?;
            let output = String::from_utf8_lossy(&plain).to_string();
            plain.zeroize();
            return Ok(Some(output));
        }

        Ok(None)
    }

    /// Retorna as notas descriptografadas de uma credencial, quando existirem.
    ///
    /// ### Parâmetros
    /// - `cred_id`: Identificador da credencial.
    ///
    /// ### Retorno
    /// - `Ok(Some(String))`: notas reveladas.
    /// - `Ok(None)`: credencial sem notas armazenadas.
    /// - `Err(anyhow)`: falha ao consultar ou descriptografar.
    pub fn reveal_notes(session: &Session, cred_id: Uuid) -> Result<Option<String>> {
        info!(
            "Solicitação de exibição de notas credencial id='{}'",
            cred_id
        );

        let cred = CredentialRepository::find_by_id(cred_id)?
            .ok_or_else(|| anyhow!("Registro não disponível no momento"))?;

        if cred.vault_id() != session.vault_id {
            return Err(anyhow!("Registro não disponível no momento"));
        }

        if let Some(cipher) = cred.notes() {
            let mut plain = decrypt_with_passphrase(&session.passphrase, cipher)?;
            let output = String::from_utf8_lossy(&plain).to_string();
            plain.zeroize();
            return Ok(Some(output));
        }

        Ok(None)
    }

    /// Pesquisa credenciais pelo nome dentro do cofre do usuário autenticado.
    ///
    /// ### Retorno
    /// - `Ok(Vec<Credential>)`: lista com os resultados encontrados.
    /// - `Err(anyhow)`: falha de consulta.
    pub fn search(session: &Session, query: &str) -> Result<Vec<Credential>> {
        info!(
            "Pesquisando credenciais vault_id='{}' termo='{}'",
            session.vault_id, query
        );
        Ok(CredentialRepository::search(session.vault_id, query)?)
    }
}

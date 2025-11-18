use anyhow::{Result, anyhow};
use rand::RngCore;
use rand_core::OsRng;
use tracing::{debug, info};
use uuid::Uuid;
use zeroize::Zeroize;

use crate::{
    infrastructure::crypto::{
        decrypt_with_passphrase, encrypt_with_passphrase, hash_password, verify_password,
    },
    models::{user::User, vault::Vault},
    repositories::{user_repository::UserRepository, vault_repository::VaultRepository},
};

/// Estrutura de sessão autenticada contendo dados necessários para operações seguras.
#[derive(Debug, Clone)]
pub struct Session {
    pub user: User,
    pub vault_id: Uuid,
    pub vault_key: Vec<u8>,
    pub passphrase: String,
}

/// Garante que informações sensíveis sejam apagadas da memória ao final da sessão.
impl Drop for Session {
    fn drop(&mut self) {
        self.vault_key.zeroize();
        self.passphrase.zeroize();
    }
}

/// Serviço responsável pelos fluxos de autenticação e registro de usuários.
pub struct AuthService;

impl AuthService {
    /// Registra um novo usuário e cria automaticamente um cofre vinculado.
    ///
    /// ### Parâmetros
    /// - `username`: Identificador textual do usuário.
    /// - `password`: Senha para derivação do hash e proteção do cofre.
    ///
    /// ### Retorno
    /// - `Ok(())` em caso de sucesso.
    /// - `Err(anyhow)` quando o nome já estiver em uso ou ocorrer falha no processo.
    ///
    /// ### Aplicação
    /// Utilizado no fluxo inicial de criação de contas, gerando o usuário e seu cofre seguro.
    pub fn register(username: &str, password: &str) -> Result<()> {
        info!(
            "Iniciando processo de registro para username='{}'",
            username
        );

        if UserRepository::find_by_username(username)?.is_some() {
            info!("Registro interrompido: username '{}' já existe", username);
            return Err(anyhow!("Nome de usuário já está em uso"));
        }

        info!("Gerando hash de senha para o novo usuário...");
        let password_hash = hash_password(password)?;

        info!("Gerando chave do cofre...");
        let mut vault_key = [0u8; 32];
        OsRng.fill_bytes(&mut vault_key);

        info!("Protegendo chave do cofre com a senha do usuário...");
        let vault_cipher = encrypt_with_passphrase(password, &vault_key)?;
        vault_key.zeroize();

        info!("Criando entidade de usuário no domínio...");
        let user = User::new(username.to_string(), password_hash)?;

        info!("Persistindo usuário no repositório...");
        UserRepository::create(&user)?;

        info!("Criando entidade de cofre no domínio...");
        let vault = Vault::new(user.id(), vault_cipher)?;

        info!("Persistindo cofre vinculado ao usuário...");
        VaultRepository::create(&vault)?;

        info!("Usuário '{}' registrado com sucesso.", username);
        Ok(())
    }

    /// Realiza autenticação de um usuário e retorna uma sessão ativa.
    ///
    /// ### Parâmetros
    /// - `username`: Nome do usuário a ser autenticado.
    /// - `password`: Senha fornecida para verificação e decriptação do cofre.
    ///
    /// ### Retorno
    /// - `Ok(Session)` quando as credenciais estiverem corretas.
    /// - `Err(anyhow)` quando o usuário não existir, a senha estiver incorreta,
    ///   ou o cofre associado não for localizado.
    ///
    /// ### Aplicação
    /// Utilizado no acesso ao sistema, retornando a chave necessária para operações
    /// criptográficas no cofre associado ao usuário autenticado.
    pub fn login(username: &str, password: &str) -> Result<Session> {
        info!("Iniciando processo de login para username='{}'", username);

        let user = match UserRepository::find_by_username(username)? {
            Some(u) => {
                info!("Usuário encontrado. Verificando credenciais...");
                u
            }
            None => {
                info!("Falha de login: usuário '{}' não encontrado", username);
                return Err(anyhow!("Usuário não encontrado"));
            }
        };

        if !verify_password(password, user.password_hash())? {
            info!(
                "Falha de login: senha incorreta para username='{}'",
                username
            );
            return Err(anyhow!("Senha incorreta"));
        }

        info!("Credenciais válidas. Localizando cofre vinculado...");
        let vault = match VaultRepository::find_by_user_id(user.id())? {
            Some(v) => v,
            None => {
                info!(
                    "Falha de login: nenhum cofre associado ao usuário '{}'",
                    username
                );
                return Err(anyhow!("Vault não encontrado para o usuário"));
            }
        };

        info!("Descriptografando chave do cofre...");
        let mut vault_key = decrypt_with_passphrase(password, vault.vault_key_cipher())?;

        info!("Sessão autenticada criada com sucesso para '{}'", username);

        let session = Session {
            user,
            vault_id: vault.id(),
            vault_key: vault_key.clone(),
            passphrase: password.to_string(),
        };

        vault_key.zeroize();
        debug!("Chave do Vault descriptografada com sucesso na memória da sessão.");
        Ok(session)
    }
}

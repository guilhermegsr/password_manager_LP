//! Módulo de Criptografia / Segurança de Senhas
//!
//! Este componente provê funcionalidades de:
//! - Derivação criptográfica de senhas (Argon2id);
//! - Validação segura de credenciais;
//! - Criptografia e descriptografia de dados sensíveis utilizando AGE com passphrase.
//!
//! Objetivo: oferecer mecanismos seguros para armazenamento e proteção
//! de informações sigilosas no contexto da aplicação.

// Dependências criptográficas e utilitárias
use age::secrecy::SecretString;
use age::{Decryptor, Encryptor};
use anyhow::{Result, anyhow};
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use std::io::{Read, Write};
use tracing::{debug, info};

/// Gera um hash criptográfico seguro para uma senha em texto puro.
///
/// Este método utiliza o algoritmo **Argon2id** com salt aleatório,
/// adequado para proteção contra ataques de força bruta e Rainbow Tables.
///
/// # Parâmetros
/// - `plain_password`: senha original em texto puro.
///
/// # Retorno
/// - `Ok(Vec<u8>)`: hash gerado no formato PHC (em bytes);
/// - `Err(anyhow::Error)`: falha durante o processo criptográfico.
///
/// # Segurança
/// O hash resultante **não permite a reversão da senha**.
pub fn hash_password(plain_password: &str) -> Result<Vec<u8>> {
    info!("Iniciando geração de hash criptográfico de senha");

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(plain_password.as_bytes(), &salt)
        .map_err(|err| anyhow!("Falha durante derivação criptográfica: {}", err))?;

    debug!("Hash criptográfico de senha gerado com sucesso");
    Ok(password_hash.to_string().into_bytes())
}

/// Valida uma senha informada contra o hash armazenado.
///
/// # Parâmetros
/// - `plain_password`: senha fornecida pelo usuário;
/// - `stored_password_hash`: hash armazenado no banco.
///
/// # Retorno
/// - `Ok(true)` caso a senha seja válida;
/// - `Ok(false)` caso seja inválida;
/// - `Err(anyhow::Error)` em falha de validação ou formatação.
///
/// # Observação
/// Utiliza Argon2id para verificação conforme práticas modernas de segurança.
pub fn verify_password(plain_password: &str, stored_password_hash: &[u8]) -> Result<bool> {
    info!("Validando credenciais fornecidas");

    let password_hash_str = std::str::from_utf8(stored_password_hash).map_err(|err| {
        anyhow!(
            "Falha ao interpretar hash criptográfico armazenado: {}",
            err
        )
    })?;

    let parsed_hash = PasswordHash::new(password_hash_str)
        .map_err(|err| anyhow!("Hash criptográfico em formato inválido: {}", err))?;

    let argon2 = Argon2::default();
    let verification_result = argon2.verify_password(plain_password.as_bytes(), &parsed_hash);

    match verification_result {
        Ok(_) => {
            debug!("Credenciais autenticadas com sucesso");
            Ok(true)
        }
        Err(_) => {
            debug!("Credenciais inválidas durante autenticação");
            Ok(false)
        }
    }
}

/// Criptografa dados sigilosos utilizando o padrão **AGE** com passphrase.
///
/// # Parâmetros
/// - `passphrase`: chave secreta de acesso;
/// - `plaintext`: dados a serem criptografados.
///
/// # Retorno
/// - `Ok(Vec<u8>)`: conteúdo cifrado pronto para persistência segura;
/// - `Err(anyhow::Error)` em caso de falha.
///
/// # Aplicação
/// Ideal para proteção de campos sensíveis como senhas,
/// notas privadas e chaves de cofres criptográficos.
pub fn encrypt_with_passphrase(passphrase: &str, plaintext: &[u8]) -> Result<Vec<u8>> {
    info!("Executando criptografia AGE com passphrase");

    let secret = SecretString::new(passphrase.to_owned());
    let encryptor = Encryptor::with_user_passphrase(secret);

    let mut encrypted_bytes: Vec<u8> = Vec::new();
    {
        let mut writer = encryptor
            .wrap_output(&mut encrypted_bytes)
            .map_err(|err| anyhow!("Falha ao iniciar processo AGE: {}", err))?;

        writer
            .write_all(plaintext)
            .map_err(|err| anyhow!("Erro ao escrever dados criptografados: {}", err))?;

        writer
            .finish()
            .map_err(|err| anyhow!("Falha ao finalizar operação AGE: {}", err))?;
    }

    debug!("Criptografia concluída com sucesso");
    Ok(encrypted_bytes)
}

/// Descriptografa dados protegidos por AGE com passphrase.
///
/// # Parâmetros
/// - `passphrase`: chave secreta de descriptografia;
/// - `ciphertext`: bytes previamente criptografados.
///
/// # Retorno
/// - `Ok(Vec<u8>)`: dados originais em texto puro;
/// - `Err(anyhow::Error)` em falhas ou passphrase incorreta.
///
/// # Observação
/// Retorna erro caso o payload não tenha sido cifrado com passphrase AGE.
pub fn decrypt_with_passphrase(passphrase: &str, ciphertext: &[u8]) -> Result<Vec<u8>> {
    info!("Executando descriptografia AGE com passphrase");

    let decryptor = Decryptor::new(ciphertext)
        .map_err(|err| anyhow!("Falha ao inicializar mecanismo AGE: {}", err))?;

    let passphrase_decryptor = match decryptor {
        Decryptor::Passphrase(d) => d,
        _ => return Err(anyhow!("Payload não foi protegido com AGE/passphrase")),
    };

    let secret = SecretString::new(passphrase.to_owned());
    let mut reader = passphrase_decryptor
        .decrypt(&secret, None)
        .map_err(|err| anyhow!("Falha na autenticação de chave AGE: {}", err))?;

    let mut decrypted_bytes = Vec::new();
    reader
        .read_to_end(&mut decrypted_bytes)
        .map_err(|err| anyhow!("Erro ao extrair conteúdo descriptografado: {}", err))?;

    debug!("Descriptografia concluída com sucesso");
    Ok(decrypted_bytes)
}

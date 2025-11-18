use anyhow::Result;
use chrono::{DateTime, Utc};
use std::time::Instant;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::infrastructure::database::get_database_connection;
use crate::models::vault::Vault;

/// Repositório responsável pela persistência e consulta de cofres criptográficos.
pub struct VaultRepository;

impl VaultRepository {
    /// Insere um cofre no banco de dados.
    ///
    /// ### Parâmetros
    /// - `vault`: Entidade `Vault` pronta para persistência.
    ///
    /// ### Retorno
    /// - `Ok(())` em caso de sucesso.
    /// - `Err(anyhow)` quando ocorrer erro na gravação.
    pub fn create(vault: &Vault) -> Result<()> {
        let start = Instant::now();
        info!(
            "Iniciando persistência do cofre. vault_id='{}' user_id='{}'",
            vault.id(),
            vault.user_id()
        );

        let conn = match get_database_connection() {
            Ok(c) => c,
            Err(err) => {
                error!("Falha ao obter conexão com banco: {}", err);
                return Err(err);
            }
        };

        let result = conn.execute(
            "INSERT INTO vault (id, user_id, vault_key_cipher, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                vault.id().as_bytes(),
                vault.user_id().as_bytes(),
                vault.vault_key_cipher(),
                vault.created_at().to_rfc3339(),
                vault.updated_at().to_rfc3339(),
            ),
        );

        match result {
            Ok(_) => {
                info!(
                    "Cofre persistido com sucesso. vault_id='{}' ({} ms)",
                    vault.id(),
                    start.elapsed().as_millis()
                );
                Ok(())
            }
            Err(err) => {
                error!("Erro ao persistir cofre vault_id='{}': {}", vault.id(), err);
                Err(err.into())
            }
        }
    }

    /// Busca um cofre pelo ID do usuário associado.
    ///
    /// ### Parâmetros
    /// - `user_id`: Identificador do usuário proprietário do cofre.
    ///
    /// ### Retorno
    /// - `Ok(Some(Vault))` quando encontrado.
    /// - `Ok(None)` quando não existir.
    /// - `Err(anyhow)` em falha de consulta ou desserialização.
    pub fn find_by_user_id(user_id: Uuid) -> Result<Option<Vault>> {
        let start = Instant::now();
        info!("Iniciando consulta de cofre por user_id='{}'", user_id);

        let conn = match get_database_connection() {
            Ok(c) => c,
            Err(err) => {
                error!("Falha ao obter conexão com banco: {}", err);
                return Err(err);
            }
        };

        let mut stmt = conn.prepare(
            "SELECT id, user_id, vault_key_cipher, created_at, updated_at
             FROM vault WHERE user_id = ?1",
        )?;

        let mut rows = stmt.query([user_id.as_bytes()])?;

        if let Some(row) = rows.next()? {
            debug!("Registro localizado para user_id='{}'", user_id);

            let vault = Vault::from_persisted(
                Uuid::from_slice(&row.get::<_, Vec<u8>>(0)?)?,
                Uuid::from_slice(&row.get::<_, Vec<u8>>(1)?)?,
                row.get(2)?,
                DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)?.with_timezone(&Utc),
                DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)?.with_timezone(&Utc),
            );

            info!(
                "Consulta concluída. Cofre encontrado para user_id='{}' ({} ms)",
                user_id,
                start.elapsed().as_millis()
            );

            return Ok(Some(vault));
        }

        warn!(
            "Nenhum cofre encontrado para user_id='{}' ({} ms)",
            user_id,
            start.elapsed().as_millis()
        );

        Ok(None)
    }
}

use anyhow::Result;
use chrono::{DateTime, Utc};
use std::time::Instant;
use tracing::{debug, error, info, trace, warn};
use uuid::Uuid;

use crate::infrastructure::database::get_database_connection;
use crate::models::credential::Credential;

/// Repositório responsável por operações de armazenamento e consulta de credenciais.
pub struct CredentialRepository;

impl CredentialRepository {
    /// Insere uma nova credencial no banco.
    ///
    /// ### Parâmetros
    /// - `credential`: Referência para a credencial já validada pelo domínio.
    ///
    /// ### Retorno
    /// - `Ok(())` quando persistida com sucesso.
    /// - `Err(anyhow)` quando ocorre falha de gravação.
    ///
    /// ### Aplicação
    /// Usado ao cadastrar uma credencial vinculada a um cofre existente.
    pub fn create(credential: &Credential) -> Result<()> {
        let start = Instant::now();
        info!(
            "Iniciando criação da credencial: id='{}', vault_id='{}', name='{}'",
            credential.id(),
            credential.vault_id(),
            credential.name()
        );

        let conn = match get_database_connection() {
            Ok(c) => c,
            Err(err) => {
                error!("Falha ao abrir conexão com banco na criação: {}", err);
                return Err(err);
            }
        };

        trace!("Executando INSERT na tabela 'credential' ...");

        let result = conn.execute(
            "INSERT INTO credential
                (id, vault_id, name, username, url, notes, password_cipher, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                credential.id().as_bytes(),
                credential.vault_id().as_bytes(),
                credential.name(),
                credential.username(),
                credential.url(),
                credential.notes(),
                credential.password_cipher(),
                credential.created_at().to_rfc3339(),
                credential.updated_at().to_rfc3339(),
            ),
        );

        match result {
            Ok(rows) => {
                info!(
                    "Credencial criada com sucesso id='{}' | linhas inseridas={} | tempo={}ms",
                    credential.id(),
                    rows,
                    start.elapsed().as_millis()
                );
                debug!(
                    "Dados persistidos: name='{}', username={:?}, url={:?}",
                    credential.name(),
                    credential.username(),
                    credential.url()
                );
                Ok(())
            }
            Err(err) => {
                error!(
                    "Erro ao criar credencial id='{}': {} | tempo={}ms",
                    credential.id(),
                    err,
                    start.elapsed().as_millis()
                );
                Err(err.into())
            }
        }
    }

    /// Atualiza os dados de uma credencial existente.
    ///
    /// ### Parâmetros
    /// - `credential`: Referência da credencial com dados já atualizados.
    ///
    /// ### Retorno
    /// - `Ok(())` mesmo quando nenhuma linha for afetada.
    /// - `Err(anyhow)` quando ocorre falha de atualização.
    ///
    /// ### Aplicação
    /// Usado quando o usuário altera dados como nome, url, notas ou senha.
    pub fn update(credential: &Credential) -> Result<()> {
        let start = Instant::now();
        info!(
            "Atualizando credencial id='{}' name='{}'",
            credential.id(),
            credential.name()
        );

        let conn = match get_database_connection() {
            Ok(c) => c,
            Err(err) => {
                error!("Falha ao abrir conexão com banco na atualização: {}", err);
                return Err(err);
            }
        };

        let now = Utc::now();

        trace!("Executando UPDATE na tabela 'credential' ...");

        let result = conn.execute(
            "UPDATE credential
                SET name = ?1, username = ?2, url = ?3, notes = ?4, password_cipher = ?5, updated_at = ?6
             WHERE id = ?7",
            (
                credential.name(),
                credential.username(),
                credential.url(),
                credential.notes(),
                credential.password_cipher(),
                now.to_rfc3339(),
                credential.id().as_bytes(),
            ),
        );

        match result {
            Ok(rows) => {
                if rows > 0 {
                    info!(
                        "Credencial atualizada com sucesso id='{}' | linhas afetadas={} | tempo={}ms",
                        credential.id(),
                        rows,
                        start.elapsed().as_millis()
                    );
                } else {
                    warn!(
                        "Atualização executada, porém nenhuma linha foi modificada. id='{}' | tempo={}ms",
                        credential.id(),
                        start.elapsed().as_millis()
                    );
                }
                Ok(())
            }
            Err(err) => {
                error!(
                    "Erro ao atualizar credencial id='{}': {} | tempo={}ms",
                    credential.id(),
                    err,
                    start.elapsed().as_millis()
                );
                Err(err.into())
            }
        }
    }

    /// Remove uma credencial pelo ID.
    ///
    /// ### Parâmetros
    /// - `id`: ID da credencial a ser removida.
    ///
    /// ### Retorno
    /// - `Ok(())` mesmo que não exista.
    /// - `Err(anyhow)` quando falha a operação de remoção.
    ///
    /// ### Aplicação
    /// Usado quando o usuário exclui permanentemente uma credencial do cofre.
    pub fn delete(id: Uuid) -> Result<()> {
        let start = Instant::now();
        info!("Removendo credencial id='{}'", id);

        let conn = match get_database_connection() {
            Ok(c) => c,
            Err(err) => {
                error!("Falha ao abrir conexão com banco na remoção: {}", err);
                return Err(err);
            }
        };

        trace!("Executando DELETE na tabela 'credential' ...");

        let result = conn.execute("DELETE FROM credential WHERE id = ?1", [id.as_bytes()]);

        match result {
            Ok(rows) => {
                if rows > 0 {
                    info!(
                        "Credencial removida com sucesso id='{}' | linhas removidas={} | tempo={}ms",
                        id,
                        rows,
                        start.elapsed().as_millis()
                    );
                } else {
                    warn!(
                        "Nenhuma credencial removida. id='{}' pode não existir. tempo={}ms",
                        id,
                        start.elapsed().as_millis()
                    );
                }
                Ok(())
            }
            Err(err) => {
                error!(
                    "Erro ao remover credencial id='{}': {} | tempo={}ms",
                    id,
                    err,
                    start.elapsed().as_millis()
                );
                Err(err.into())
            }
        }
    }

    /// Busca uma credencial pelo ID.
    pub fn find_by_id(id: Uuid) -> Result<Option<Credential>> {
        debug!("Consultando credencial por id='{}'", id);

        let conn = get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, vault_id, name, username, url, notes, password_cipher, created_at, updated_at
             FROM credential WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id.as_bytes()])?;
        if let Some(row) = rows.next()? {
            trace!("Registro encontrado, convertendo linha em Credencial...");
            return Ok(Some(Self::row_to_model(row)?));
        }

        warn!("Nenhuma credencial encontrada com id='{}'", id);
        Ok(None)
    }

    /// Lista todas as credenciais pertencentes a um cofre.
    pub fn find_all_by_vault_id(vault_id: Uuid) -> Result<Vec<Credential>> {
        debug!("Listando credenciais para vault_id='{}'", vault_id);

        let conn = get_database_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, vault_id, name, username, url, notes, password_cipher, created_at, updated_at
             FROM credential WHERE vault_id = ?1",
        )?;

        let rows = stmt.query_map([vault_id.as_bytes()], |row| Self::row_to_model(row))?;
        let list: rusqlite::Result<Vec<_>> = rows.collect();

        info!(
            "Consulta finalizada. Registros retornados={}",
            list.as_ref().map(|v| v.len()).unwrap_or(0)
        );

        Ok(list?)
    }

    /// Pesquisa credenciais pelo nome.
    pub fn search(vault_id: Uuid, query: &str) -> Result<Vec<Credential>> {
        debug!(
            "Pesquisando credenciais: vault_id='{}', termo='{}'",
            vault_id, query
        );

        let conn = get_database_connection()?;
        let pattern = format!("%{}%", query);

        let mut stmt = conn.prepare(
            "SELECT id, vault_id, name, username, url, notes, password_cipher, created_at, updated_at
             FROM credential
             WHERE vault_id = ?1 AND name LIKE ?2
             ORDER BY name ASC",
        )?;

        let rows = stmt.query_map(rusqlite::params![vault_id.as_bytes(), pattern], |row| {
            Self::row_to_model(row)
        })?;

        let list: rusqlite::Result<Vec<_>> = rows.collect();

        info!(
            "Pesquisa concluída para termo='{}'. Resultados={}",
            query,
            list.as_ref().map(|v| v.len()).unwrap_or(0)
        );

        Ok(list?)
    }

    /// Converte uma linha SQL em objeto de domínio.
    ///
    /// ### Aplicação
    /// Uso interno do repositório durante consultas.
    fn row_to_model(row: &rusqlite::Row) -> rusqlite::Result<Credential> {
        trace!("Convertendo linha SQL em Credential ...");

        let id = Uuid::from_slice(&row.get::<_, Vec<u8>>(0)?).map_err(|e| {
            error!("Falha ao converter UUID(id) a partir do banco: {}", e);
            rusqlite::Error::FromSqlConversionFailure(16, rusqlite::types::Type::Blob, Box::new(e))
        })?;

        let vault_id = Uuid::from_slice(&row.get::<_, Vec<u8>>(1)?).map_err(|e| {
            error!("Falha ao converter UUID(vault_id) a partir do banco: {}", e);
            rusqlite::Error::FromSqlConversionFailure(16, rusqlite::types::Type::Blob, Box::new(e))
        })?;

        let created_at = DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
            .map_err(|e| {
                error!(
                    "Falha ao converter created_at (RFC3339) a partir do banco: {}",
                    e
                );
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?
            .with_timezone(&Utc);

        let updated_at = DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
            .map_err(|e| {
                error!(
                    "Falha ao converter updated_at (RFC3339) a partir do banco: {}",
                    e
                );
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?
            .with_timezone(&Utc);

        debug!(
            "Linha convertida em Credential {{ id='{}', vault_id='{}' }}",
            id, vault_id
        );

        Ok(Credential::from_persisted(
            id,
            vault_id,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
            row.get(5)?,
            row.get(6)?,
            created_at,
            updated_at,
        ))
    }
}

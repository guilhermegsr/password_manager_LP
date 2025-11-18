use crate::infrastructure::database::get_database_connection;
use crate::models::user::User;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use std::time::Instant;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Repositório responsável por operações de persistência e consulta de usuários.
pub struct UserRepository;

impl UserRepository {
    /// Insere um usuário no banco de dados.
    ///
    /// ### Parâmetros
    /// - `user`: Referência para a entidade de domínio já validada.
    ///
    /// ### Retorno
    /// - `Ok(())` em sucesso.
    /// - `Err(anyhow)` em falha de gravação.
    pub fn create(user: &User) -> Result<()> {
        let start = Instant::now();
        info!(
            "Iniciando persistência do usuário. username='{}' id='{}'",
            user.username(),
            user.id()
        );

        let conn = match get_database_connection() {
            Ok(c) => c,
            Err(err) => {
                error!(
                    "Falha ao obter conexão com banco ao criar usuário '{}': {}",
                    user.username(),
                    err
                );
                return Err(err);
            }
        };

        let result = conn.execute(
            "INSERT INTO user (id, username, password_hash, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                user.id().as_bytes(),
                user.username(),
                user.password_hash(),
                user.created_at().to_rfc3339(),
                user.updated_at().to_rfc3339(),
            ),
        );

        match result {
            Ok(_) => {
                let duration = start.elapsed();
                info!(
                    "Usuário '{}' persistido com sucesso. ({} ms)",
                    user.username(),
                    duration.as_millis()
                );
                debug!(
                    "Detalhes de inserção: id='{}', created_at='{}', updated_at='{}'",
                    user.id(),
                    user.created_at(),
                    user.updated_at()
                );
                Ok(())
            }
            Err(err) => {
                let duration = start.elapsed();
                error!(
                    "Erro ao persistir usuário '{}' após {} ms: {}",
                    user.username(),
                    duration.as_millis(),
                    err
                );
                Err(err.into())
            }
        }
    }

    /// Busca um usuário pelo seu nome de login.
    ///
    /// ### Parâmetros
    /// - `username`: Nome de usuário desejado.
    ///
    /// ### Retorno
    /// - `Ok(Some(User))` se encontrado.
    /// - `Ok(None)` quando não existe.
    /// - `Err(anyhow)` quando ocorre erro de consulta ou parsing de dados.
    pub fn find_by_username(username: &str) -> Result<Option<User>> {
        let start = Instant::now();
        info!("Iniciando consulta de usuário pelo username='{}'", username);

        let conn = match get_database_connection() {
            Ok(c) => c,
            Err(err) => {
                error!(
                    "Falha ao obter conexão com banco ao consultar username='{}': {}",
                    username, err
                );
                return Err(err);
            }
        };

        let mut stmt = conn.prepare(
            "SELECT id, username, password_hash, created_at, updated_at
             FROM user WHERE username = ?1",
        )?;

        let mut rows = stmt.query([username])?;

        if let Some(row) = rows.next()? {
            debug!("Registro localizado para username='{}'", username);

            let id_bytes: Vec<u8> = row.get(0)?;
            let id = Uuid::from_slice(&id_bytes)
                .map_err(|_| anyhow!("UUID inválido no banco de dados"))?;

            let username: String = row.get(1)?;
            let password_hash: Vec<u8> = row.get(2)?;

            let created_at_str: String = row.get(3)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);

            let updated_at_str: String = row.get(4)?;
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);

            let duration = start.elapsed();
            info!(
                "Consulta concluída. Usuário encontrado username='{}' ({} ms)",
                username,
                duration.as_millis()
            );

            debug!(
                "Dados carregados: id='{}', created_at='{}', updated_at='{}'",
                id, created_at, updated_at
            );

            let user = User::from_persisted(id, username, password_hash, created_at, updated_at);
            Ok(Some(user))
        } else {
            let duration = start.elapsed();
            warn!(
                "Usuário não encontrado para username='{}' ({} ms)",
                username,
                duration.as_millis()
            );
            Ok(None)
        }
    }
}

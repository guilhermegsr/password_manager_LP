//! Módulo de acesso ao banco de dados SQLite
//!
//! Responsável por inicializar conexão, aplicar migrações e configurar
//! parâmetros fundamentais de segurança e integridade transacional.

use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::{env, fs, path::Path};
use tracing::{debug, info};

/// Conteúdo SQL inicial da migração do banco.
static DATABASE_MIGRATIONS: Lazy<&'static str> =
    Lazy::new(|| include_str!("../../migrations/001_init.sql"));

/// Retorna uma conexão SQLite pronta para uso, aplicando migrações
/// automaticamente na primeira execução.
///
/// O caminho do banco é lido da variável:
/// ```env
/// DATABASE_URL="data/vault.db"
/// ```
pub fn get_database_connection() -> Result<Connection> {
    info!("Obtendo nova conexão com o banco de dados");

    let db_url =
        env::var("DATABASE_URL").context("Variável de ambiente DATABASE_URL não foi definida")?;

    // Cria pasta caso não exista
    if let Some(parent) = Path::new(&db_url).parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Falha ao criar diretórios necessários para armazenamento do banco: {:?}",
                parent
            )
        })?;
    }

    let is_first_creation = !Path::new(&db_url).exists();

    debug!("Abrindo banco SQLite em {}", db_url);
    let connection =
        Connection::open(&db_url).with_context(|| format!("Falha ao abrir banco em {}", db_url))?;

    // Configuração de parâmetros recomendados para o SQLite:
    //
    // - PRAGMA foreign_keys = ON
    //   Garante integridade referencial entre tabelas, impedindo remoções/alterações
    //   que violem relações definidas (ex.: credenciais órfãs sem vault).
    //
    // - PRAGMA journal_mode = WAL
    //   Habilita Write-Ahead Logging, proporcionando maior segurança contra corrupção
    //   de dados e melhor desempenho em cenários com múltiplas leituras/gravações.

    debug!("Ativando PRAGMA foreign_keys = ON");
    connection
        .pragma_update(None, "foreign_keys", &"ON")
        .context("Falha ao ativar PRAGMA foreign_keys")?;

    debug!("Ativando PRAGMA journal_mode = WAL");
    connection
        .pragma_update(None, "journal_mode", &"WAL")
        .context("Falha ao ativar PRAGMA journal_mode")?;

    // Executa migrações apenas na primeira criação do banco
    if is_first_creation {
        info!("Banco criado pela primeira vez, executando migrações...");
        apply_migrations(&connection)?;
        info!("Migrações concluídas com sucesso.");
    }

    Ok(connection)
}

/// Executa o script SQL de migrações iniciais.
fn apply_migrations(connection: &Connection) -> Result<()> {
    info!("Executando migrações do banco de dados");

    debug!("Rodando script SQL de migração inicial");
    connection
        .execute_batch(*DATABASE_MIGRATIONS)
        .context("Erro ao executar migrações SQL")?;

    Ok(())
}

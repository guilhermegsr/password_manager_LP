/*!
Inicializa o sistema de logging da aplicação.

Este arquivo configura o `tracing` para registrar eventos com níveis configuráveis
via variáveis de ambiente, permitindo rastreamento de execução em modo de produção
ou desenvolvimento, com saída em arquivo e/ou terminal.

Variáveis de ambiente suportadas:
- LOG_LEVEL: Define o nível mínimo de log (error, warn, info, debug, trace).
             Valor padrão: "info".
- LOG_FILE_ONLY: Quando "true", envia logs somente para o arquivo.
                 Quando "false", envia logs para arquivo e console (stdout).
                 Valor padrão: "false".
- LOG_FILE_PATH: Define o caminho completo do arquivo de log.
                 Valor padrão: "logs/password_manager.log".
*/

use std::{env, fs};
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Inicializa o registrador de logs da aplicação.
pub fn init_logger() {
    // Obtém o caminho do arquivo de log ou usa padrão.
    let log_file_path =
        env::var("LOG_FILE_PATH").unwrap_or_else(|_| "logs/password_manager.log".to_string());

    // Cria o diretório base, se existir na configuração.
    if let Some(parent) = std::path::Path::new(&log_file_path).parent() {
        fs::create_dir_all(parent).ok();
    }

    // Obtém o nível de logging via ambiente.
    let log_level_env = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let level = match log_level_env.to_lowercase().as_str() {
        "error" => Level::ERROR,
        "warn" | "warning" => Level::WARN,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    };

    // Obtém configuração se logs devem ir somente para arquivo.
    let log_file_only = env::var("LOG_FILE_ONLY")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);

    // Define filtro de nível.
    let filter_layer = EnvFilter::default().add_directive(level.into());

    // Abre o arquivo de log.
    let file_writer = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .expect("Não foi possível criar o arquivo de log especificado.");

    // Layer de log em arquivo.
    let file_layer = fmt::layer()
        .with_target(false)
        .with_line_number(true)
        .compact()
        .with_writer(file_writer);

    // Layer de log em console.
    let console_layer = fmt::layer().with_target(false).compact();

    let registry = tracing_subscriber::registry()
        .with(filter_layer)
        .with(file_layer);

    if log_file_only {
        registry.init();
    } else {
        registry.with(console_layer).init();
    }
}

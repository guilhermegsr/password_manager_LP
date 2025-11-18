mod infrastructure;
mod models;
mod repositories;
mod services;

use std::{
    env,
    io::{self, Write},
};
use tracing::{info, warn};
use uuid::Uuid;

use infrastructure::{database::get_database_connection, logger::init_logger};
use services::{auth_service::AuthService, credential_service::CredentialService};

/// Função auxiliar para entrada de dados via CLI.
fn input(prompt: &str) -> String {
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() -> anyhow::Result<()> {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
    let env_file = format!(".env.{}", app_env);

    match dotenvy::from_filename(&env_file) {
        Ok(_) => println!("Usando ambiente: {app_env} ({env_file})"),
        Err(_) => println!("Aviso: {env_file} não encontrado, seguindo sem .env específico"),
    }

    init_logger();

    let _ = get_database_connection()?;
    info!("Password Manager iniciado no modo '{app_env}'");

    loop {
        println!("\n=== Password Manager CLI ===");
        println!("[1] Criar usuário");
        println!("[2] Login");
        println!("[0] Sair");

        match input("Escolha: ").as_str() {
            "1" => {
                let user = input("Novo usuário: ");
                let pass = input("Senha: ");
                match AuthService::register(&user, &pass) {
                    Ok(_) => info!("Usuário criado com sucesso."),
                    Err(e) => warn!("Falha ao criar usuário: {e}"),
                }
            }
            "2" => {
                let user = input("Usuário: ");
                let pass = input("Senha: ");
                match AuthService::login(&user, &pass) {
                    Ok(session) => menu_credenciais(session)?,
                    Err(e) => warn!("Falha no login: {e}"),
                }
            }
            "0" => {
                println!("Saindo...");
                return Ok(());
            }
            _ => println!("Opção inválida."),
        }
    }
}

/// Submenu de operações relacionadas às credenciais do cofre do usuário logado.
fn menu_credenciais(session: services::auth_service::Session) -> anyhow::Result<()> {
    loop {
        println!("\n=== Menu de Credenciais ===");
        println!("[1] Criar credencial");
        println!("[2] Listar credenciais");
        println!("[3] Buscar credenciais por nome");
        println!("[4] Mostrar credencial completa");
        println!("[5] Atualizar credencial");
        println!("[6] Remover credencial");
        println!("[0] Logout");

        match input("Escolha: ").as_str() {
            "1" => {
                let name = input("Nome da credencial: ");
                let user = input("Username (opcional): ");
                let url = input("URL (opcional): ");
                let notes = input("Notas (opcional): ");
                let pwd = input("Senha (opcional): ");

                match CredentialService::create(
                    &session,
                    &name,
                    if user.is_empty() { None } else { Some(user) },
                    if url.is_empty() { None } else { Some(url) },
                    if notes.is_empty() {
                        None
                    } else {
                        Some(notes.into_bytes())
                    },
                    if pwd.is_empty() { None } else { Some(&pwd) },
                ) {
                    Ok(_) => println!("Credencial criada!"),
                    Err(e) => warn!("Erro ao criar credencial: {e}"),
                }
            }

            "2" => {
                let list = CredentialService::list(&session)?;
                println!("\nCredenciais:");
                for c in list {
                    println!("→ {} ({})", c.name(), c.id());
                }
            }

            "3" => {
                let q = input("Buscar por nome: ");
                let results = CredentialService::search(&session, &q)?;
                println!("\nResultados:");
                for c in results {
                    println!("→ {} ({})", c.name(), c.id());
                }
            }

            "4" => {
                let id = input("ID da credencial: ");
                let Ok(uuid) = Uuid::parse_str(&id) else {
                    println!("UUID inválido.");
                    continue;
                };

                match CredentialService::get(&session, uuid) {
                    Ok(cred) => {
                        println!("\n--- Credencial ---");
                        println!("Nome: {}", cred.name());

                        if let Some(u) = cred.username() {
                            println!("Usuário: {u}");
                        }
                        if let Some(url) = cred.url() {
                            println!("URL: {url}");
                        }

                        match CredentialService::reveal_notes(&session, uuid) {
                            Ok(Some(n)) => println!("Notas: {n}"),
                            Ok(None) => println!("Sem notas armazenadas."),
                            Err(e) => println!("Erro ao descriptografar notas: {e}"),
                        }

                        match CredentialService::reveal_password(&session, uuid) {
                            Ok(Some(p)) => println!("Senha: {p}"),
                            Ok(None) => println!("Sem senha armazenada."),
                            Err(e) => println!("Erro ao descriptografar senha: {e}"),
                        }
                    }
                    Err(e) => println!("Erro: {e}"),
                }
            }

            "5" => {
                let id = input("ID da credencial: ");
                let Ok(uuid) = Uuid::parse_str(&id) else {
                    println!("UUID inválido.");
                    continue;
                };

                match CredentialService::get(&session, uuid) {
                    Ok(cred) => {
                        println!("\n--- Atualizar Credencial ---");

                        let newname = input("Novo nome (vazio = manter): ");
                        let newuser = input("Novo username (vazio = manter): ");
                        let newurl = input("Nova URL (vazio = manter): ");
                        let newnotes = input("Novas notas (vazio = manter): ");
                        let newpwd = input("Nova senha (vazio = manter): ");

                        match CredentialService::update(
                            &session,
                            cred,
                            if newname.is_empty() {
                                None
                            } else {
                                Some(newname)
                            },
                            if newuser.is_empty() {
                                None
                            } else {
                                Some(newuser)
                            },
                            if newurl.is_empty() {
                                None
                            } else {
                                Some(newurl)
                            },
                            if newnotes.is_empty() {
                                None
                            } else {
                                Some(newnotes.into_bytes())
                            },
                            if newpwd.is_empty() {
                                None
                            } else {
                                Some(newpwd.as_str())
                            },
                        ) {
                            Ok(_) => println!("Credencial atualizada."),
                            Err(e) => println!("Erro ao atualizar: {e}"),
                        }
                    }
                    Err(_) => println!("Credencial não encontrada."),
                }
            }

            "6" => {
                let id = input("ID da credencial: ");
                let Ok(uuid) = Uuid::parse_str(&id) else {
                    println!("UUID inválido.");
                    continue;
                };

                match CredentialService::delete(&session, uuid) {
                    Ok(_) => println!("Operação concluída."),
                    Err(e) => println!("Falha na operação: {e}"),
                }
            }

            "0" => {
                println!("Logout realizado.");
                return Ok(());
            }

            _ => println!("Opção inválida."),
        }
    }
}

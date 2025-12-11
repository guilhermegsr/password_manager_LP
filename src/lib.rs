//! # Password Manager
//!
//! Este crate implementa o núcleo de um **gerenciador de senhas seguro**, com foco em
//! **segurança de dados**, **separação de responsabilidades** e **arquitetura em camadas**.
//!
//! A biblioteca fornece toda a lógica de domínio utilizada por um executável CLI,
//! incluindo autenticação de usuários, gerenciamento de credenciais e persistência segura.
//!
//! ---
//!
//! ## Objetivo
//!
//! Fornecer uma base reutilizável e testável para aplicações que necessitam armazenar,
//! proteger e manipular credenciais sensíveis, como senhas, URLs e anotações privadas,
//! respeitando boas práticas de segurança.
//!
//! ---
//!
//! ## Arquitetura
//!
//! O projeto é organizado em camadas bem definidas:
//!
//! ```text
//! ┌─────────────┐
//! │   CLI/App   │  ← Interface de usuário (main.rs)
//! └─────────────┘
//!        │
//! ┌─────────────┐
//! │  Services   │  ← Regras de negócio e casos de uso
//! └─────────────┘
//!        │
//! ┌─────────────┐
//! │ Repositories│  ← Persistência e acesso a dados
//! └─────────────┘
//!        │
//! ┌─────────────┐
//! │ Infrastructure │ ← Banco, logging, configuração
//! └─────────────┘
//!        │
//! ┌─────────────┐
//! │   Models    │  ← Entidades e tipos de domínio
//! └─────────────┘
//! ```
//!
//! Essa separação facilita manutenção, testes e possível reutilização da biblioteca
//! em outros contextos (ex.: API HTTP ou GUI).
//!
//! ---
//!
//! ## Módulos
//!
//! ### [`infrastructure`]
//!
//! Camada responsável por detalhes técnicos:
//!
//! - Conexão com o banco de dados
//! - Inicialização de logging
//! - Integração com variáveis de ambiente
//!
//! Essa camada **não contém regras de negócio**.
//!
//! ---
//!
//! ### [`models`]
//!
//! Define as **entidades do domínio**, como:
//!
//! - Usuários
//! - Sessões
//! - Credenciais
//!
//! Os modelos representam o estado central da aplicação.
//!
//! ---
//!
//! ### [`repositories`]
//!
//! Camada de persistência:
//!
//! - Acesso ao banco de dados
//! - CRUD de usuários e credenciais
//! - Isolamento da fonte de dados
//!
//! Os services dependem desta camada, e não diretamente da infraestrutura.
//!
//! ---
//!
//! ### [`services`]
//!
//! Implementa as **regras de negócio** da aplicação:
//!
//! - Autenticação e gerenciamento de sessões
//! - Criação, leitura, atualização e remoção de credenciais
//! - Controle de acesso e validações
//!
//! Serviços importantes incluem:
//!
//! - [`services::auth_service::AuthService`]
//! - [`services::credential_service::CredentialService`]
//!
//! ---
//!
//! ## Fluxo típico de uso
//!
//! ```text
//! Usuário → AuthService → Session
//!        → CredentialService (com Session válida)
//! ```
//!
//! O acesso às credenciais exige sempre uma sessão autenticada,
//! garantindo isolamento entre usuários.
//!
//! ---
//!
//! ## Segurança
//!
//! O crate foi projetado para:
//!
//! - Evitar exposição acidental de dados sensíveis
//! - Centralizar operações de criptografia/descriptografia
//! - Isolar credenciais por usuário autenticado
//!
//! Detalhes sensíveis **não são manipulados diretamente pela interface de usuário**.
//!
//! ---
//!
//! ## Uso como biblioteca
//!
//! Embora o projeto possua um executável CLI, este crate pode ser reutilizado por
//! outras interfaces, como APIs REST ou aplicações gráficas.
//!
//! ---
//!
//! ## Público-alvo
//!
//! - Pessoas que demandam gerenciamento seguro de credenciais
//!


pub mod infrastructure;
pub mod models;
pub mod repositories;
pub mod services;

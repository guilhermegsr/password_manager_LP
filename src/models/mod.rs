//! # Models
//!
//! Este módulo define os **modelos de domínio** do gerenciador de senhas.
//!
//! Os modelos representam as **entidades centrais da aplicação**, descrevendo
//! dados, identidade e invariantes fundamentais do sistema, independentemente
//! de infraestrutura ou interface de usuário.
//!
//! ---
//!
//! ## Responsabilidades
//!
//! O módulo `models` é responsável por:
//!
//! - Definir as estruturas de dados principais do domínio
//! - Representar identidades, como usuários, cofres e credenciais
//! - Garantir coerência estrutural das entidades
//!
//! Regras de negócio **não devem** ser implementadas diretamente neste módulo;
//! elas pertencem à camada de `services`.
//!
//! ---
//!
//! ## Submódulos
//!
//! ### [`user`]
//!
//! Representa usuários do sistema:
//!
//! - Identidade do usuário
//! - Informações necessárias para autenticação
//! - Associação com cofres ou sessões
//!
//! ---
//!
//! ### [`vault`]
//!
//! Representa o cofre de credenciais de um usuário:
//!
//! - Isolamento lógico de credenciais
//! - Associação direta a um único usuário
//! - Contêiner lógico para dados sensíveis
//!
//! ---
//!
//! ### [`credential`]
//!
//! Representa uma credencial armazenada no cofre:
//!
//! - Nome descritivo
//! - Dados opcionais (username, URL, notas, senha)
//! - Identidade única para referência e persistência
//!
//! ---
//!
//! ## Princípios de design
//!
//! - **Modelo rico, porém passivo**: entidades carregam dados e invariantes,
//!   enquanto regras de negócio ficam nos serviços
//! - **Clareza semântica**: cada modelo representa um conceito explícito do domínio
//! - **Independência de infraestrutura**: modelos não conhecem banco, criptografia
//!
//! ---
//!
//! ## Uso na aplicação
//!
//! Os modelos são utilizados por:
//!
//! - Repositórios, para persistência
//! - Serviços, para aplicar regras de negócio
//!
//! Eles não devem ser manipulados diretamente pela camada de interface (CLI).
//!
//! ---
//!
//! ## Observação
//!
//! Qualquer alteração neste módulo impacta diretamente o domínio do sistema
//! e deve preservar compatibilidade conceitual e estrutural.


pub mod user;
pub mod vault;
pub mod credential;
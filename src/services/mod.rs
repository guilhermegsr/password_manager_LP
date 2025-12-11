//! # Services
//!
//! Este módulo implementa a **camada de serviços da aplicação**, concentrando
//! as **regras de negócio** e os **casos de uso** do gerenciador de senhas.
//!
//! Os serviços coordenam o fluxo entre modelos, repositórios e infraestrutura,
//! garantindo que todas as operações respeitem as regras do domínio e os
//! requisitos de segurança.
//!
//! ---
//!
//! ## Responsabilidades
//!
//! O módulo `services` é responsável por:
//!
//! - Aplicar regras de negócio do sistema
//! - Coordenar operações entre repositórios
//! - Validar fluxos de autenticação e acesso
//!
//! Esta camada **não realiza entrada/saída direta com o usuário**
//! e **não conhece detalhes da interface CLI**.
//!
//! ---
//!
//! ## Submódulos
//!
//! ### [`auth_service`]
//!
//! Responsável por autenticação e controle de acesso:
//!
//! - Registro de novos usuários
//! - Validação de credenciais de login
//! - Criação e gerenciamento de sessões autenticadas
//!
//! Este serviço garante que apenas usuários autenticados tenham acesso
//! às operações sensíveis.
//!
//! ---
//!
//! ### [`credential_service`]
//!
//! Responsável pela gestão de credenciais dentro do cofre do usuário:
//!
//! - Criação de credenciais
//! - Listagem e busca
//! - Atualização de dados sensíveis
//! - Remoção de credenciais
//!
//! Todas as operações exigem uma **sessão válida**, garantindo
//! isolamento e segurança entre usuários.
//!
//! ---
//!
//! ## Fluxo de uso típico
//!
//! ```text
//! Interface (CLI)
//!     │
//!     ▼
//! AuthService
//!     │
//!     ▼
//! CredentialService
//! ```
//!
//! Os serviços atuam como **fachadas** para as operações do sistema,
//! simplificando o uso pela interface.
//!
//! ---
//!
//! ## Princípios de design
//!
//! - **Centralização das regras de negócio**
//! - **Validação consistente** antes de qualquer operação
//! - **Baixo acoplamento** com infraestrutura
//! - **Orientação a casos de uso**
//!
//! ---
//!
//! ## Uso esperado
//!
//! Este módulo deve ser utilizado por:
//!
//! - Interfaces de usuário (CLI, API, GUI)
//!
//! Ele **não deve ser acessado diretamente** por testes de infraestrutura
//! nem por código de baixo nível.
//!
//! ---
//!
//! ## Observação
//!
//! Alterações nos serviços impactam diretamente o comportamento da aplicação
//! e devem preservar invariantes de segurança e domínio.

pub mod auth_service;
pub mod credential_service;

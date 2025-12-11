//! # Repositories
//!
//! Este módulo implementa a **camada de persistência** do gerenciador de senhas.
//!
//! Os repositórios são responsáveis por **acessar, armazenar e recuperar**
//! entidades do domínio, isolando completamente o restante da aplicação
//! dos detalhes do banco de dados e do mecanismo de armazenamento.
//!
//! ---
//!
//! ## Responsabilidades
//!
//! O módulo `repositories` é responsável por:
//!
//! - Persistir entidades do domínio
//! - Executar operações de leitura e escrita no banco
//! - Encapsular consultas e comandos de persistência
//!
//! Essa camada **não contém regras de negócio** e **não executa validações**
//! além das estritamente necessárias para acesso a dados.
//!
//! ---
//!
//! ## Submódulos
//!
//! ### [`user_repository`]
//!
//! Gerencia a persistência de usuários:
//!
//! - Criação e recuperação de usuários
//! - Consulta por identificadores únicos
//! - Verificação de existência
//!
//! ---
//!
//! ### [`vault_repository`]
//!
//! Gerencia a persistência dos cofres de usuários:
//!
//! - Associação entre usuário e cofre
//! - Recuperação do cofre pertencente a um usuário
//! - Isolamento lógico entre cofres
//!
//! ---
//!
//! ### [`credential_repository`]
//!
//! Gerencia a persistência das credenciais:
//!
//! - Armazenamento de credenciais no cofre
//! - Busca, listagem e remoção de credenciais
//! - Operações por identificador único (UUID)
//!
//! ---
//!
//! ## Relação com outras camadas
//!
//! - Depende de [`crate::infrastructure`] para acesso ao banco de dados
//! - Opera sobre entidades definidas em [`crate::models`]
//! - É utilizada exclusivamente pelos [`crate::services`]
//!
//! A interface de usuário **não deve acessar repositórios diretamente**.
//!
//! ---
//!
//! ## Princípios de design
//!
//! - **Isolamento de persistência**: detalhes do banco não vazam para o domínio
//! - **Baixo acoplamento**: services não conhecem SQL ou conexões
//! - **Testabilidade**: repositórios podem ser simulados ou substituídos
//!
//! ---
//!
//! ## Observação
//!
//! Alterações na estrutura dos repositórios podem impactar a integridade
//! dos dados e devem ser realizadas com atenção.

pub mod credential_repository;
pub mod user_repository;
pub mod vault_repository;

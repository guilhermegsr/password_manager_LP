//! # Infrastructure
//!
//! Este módulo concentra os **detalhes técnicos e de infraestrutura**
//! do gerenciador de senhas.
//!
//! Ele fornece implementações concretas para serviços de baixo nível,
//! isolando a aplicação de aspectos como banco de dados, criptografia
//! e logging.
//!
//! ---
//!
//! ## Responsabilidades
//!
//! O módulo `infrastructure` é responsável por:
//!
//! - Conexão e configuração do banco de dados
//! - Inicialização e configuração do sistema de logs
//! - Operações criptográficas de baixo nível
//!
//! Essa camada **não contém regras de negócio** e **não deve depender**
//! dos módulos de `services` ou `models`.
//!
//! ---
//!
//! ## Submódulos
//!
//! ### [`database`]
//!
//! Gerencia o acesso ao banco de dados:
//!
//! - Criação e reaproveitamento de conexões
//! - Inicialização do schema
//! - Encapsulamento da camada de persistência
//!
//! ---
//!
//! ### [`crypto`]
//!
//! Fornece primitivas criptográficas utilizadas pela aplicação:
//!
//! - Hashing de senhas
//! - Criptografia e descriptografia de dados sensíveis
//! - Geração e manipulação de chaves
//!
//! Todas as decisões criptográficas ficam centralizadas neste módulo,
//! reduzindo o risco de uso incorreto.
//!
//! ---
//!
//! ### [`logger`]
//!
//! Inicializa e configura o sistema de logging:
//!
//! - Integração com o crate `tracing`
//! - Definição de níveis de log por ambiente
//! - Saída padronizada para diagnóstico e auditoria
//!
//! ---
//!
//! ## Princípios de design
//!
//! - **Isolamento técnico**: detalhes de infraestrutura não vazam para o domínio
//! - **Substituibilidade**: banco ou logger podem ser alterados sem afetar serviços
//! - **Responsabilidade única**: cada submódulo possui um papel bem definido
//!
//! ---
//!
//! ## Uso interno
//!
//! Este módulo é utilizado principalmente pelos repositórios e serviços
//! da aplicação e **não é voltado para uso direto pela interface CLI**.
//!
//! ---
//!
//! ## Observação
//!
//! Alterações neste módulo devem ser feitas com cautela, pois impactam
//! diretamente a segurança e estabilidade do sistema.


pub mod crypto;
pub mod database;
pub mod logger;

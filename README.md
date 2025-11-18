# Password Manager

## Descrição

Gerenciador de senhas local executado via linha de comando (CLI), desenvolvido em Rust. Todos os dados são armazenados
de forma local utilizando SQLite, com senhas e notas criptografadas. Nenhum dado é enviado para qualquer serviço
externo.

## Funcionalidades

* Cadastro de usuário
* Autenticação e abertura de cofre
* Criação, listagem, busca, atualização e remoção de credenciais
* Armazenamento de senhas e notas criptografadas

## Tecnologias Principais

| Categoria                | Biblioteca                  |
|--------------------------|-----------------------------|
| Banco de dados           | rusqlite                    |
| Criptografia             | age, argon2, zeroize        |
| Identidade e datas       | uuid, chrono                |
| Configuração de ambiente | dotenvy                     |
| Logging                  | tracing, tracing-subscriber |

## Requisitos

* Rust 1.72 ou superior

## Instalação

```
cargo build
```

## Configuração de Ambiente

A aplicação carrega automaticamente o arquivo `.env` conforme a variável `APP_ENV`.

Exemplos de arquivos:

`.env.development`:

```
RUST_LOG=debug
DATABASE_URL=data/dev.db
```

`.env.production`:

```
RUST_LOG=info
DATABASE_URL=data/prod.db
```

## Execução

Ambiente de desenvolvimento (padrão):

```
cargo run
```

Ambiente de produção:

```
APP_ENV=production cargo run
```

## Estrutura do Banco

| Entidade   | Campos                                                                                |
|------------|---------------------------------------------------------------------------------------|
| User       | id, username, password_hash, created_at, updated_at                                   |
| Vault      | id, user_id, vault_key_cipher, created_at, updated_at                                 |
| Credential | id, vault_id, name, username?, url?, notes?, password_cipher?, created_at, updated_at |

## Fluxo de Uso

Menu inicial:

```
[1] Criar usuário
[2] Login
[0] Sair
```

Após login:

```
[1] Criar credencial
[2] Listar
[3] Buscar
[4] Exibir
[5] Atualizar
[6] Remover
[0] Logout
```

## Observações

* Senhas não são armazenadas em texto plano
* A chave do cofre permanece cifrada
* Dados sensíveis são removidos da memória com `zeroize`
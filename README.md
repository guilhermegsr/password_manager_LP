# Password Manager

## Visão Geral

Password Manager é um gerenciador de senhas local, desenvolvido em Rust, que pode ser utilizado tanto via linha de comando (CLI) quanto por uma interface gráfica desktop construída com Tauri e React. Todos os dados são armazenados exclusivamente no ambiente local do usuário, utilizando SQLite, com senhas e informações sensíveis devidamente criptografadas. Nenhum dado é transmitido para serviços externos.

---

## Funcionalidades

* Cadastro de usuários
* Autenticação segura e abertura de cofre criptografado
* Criação, listagem, busca, atualização e remoção de credenciais
* Armazenamento criptografado de senhas e notas
* Execução via CLI ou aplicação desktop

---

## Tecnologias Utilizadas

| Camada             | Tecnologias / Bibliotecas   |
| ------------------ | --------------------------- |
| Banco de dados     | rusqlite                    |
| Criptografia       | age, argon2, zeroize        |
| Identidade e datas | uuid, chrono                |
| Configuração       | dotenvy                     |
| Logging            | tracing, tracing-subscriber |
| Interface gráfica  | Tauri, React, Vite          |

---

## Requisitos

* Rust 1.72 ou superior
* Node.js
* Bun
* Dependências do sistema necessárias para o Tauri

---

## Configuração de Ambiente

A aplicação carrega automaticamente o arquivo `.env` de acordo com o ambiente definido.

Exemplo de `.env.development`:

```env
RUST_LOG=debug
DATABASE_URL=data/dev.db
```

Exemplo de `.env.production`:

```env
RUST_LOG=info
DATABASE_URL=data/prod.db
```

---

## Execução

### Backend (CLI)

Ambiente de desenvolvimento:

```bash
cargo run
```

Ambiente de produção:

```bash
APP_ENV=production cargo run
```

---

### Aplicação Desktop (Frontend + Backend)

No diretório da interface gráfica:

```bash
bun run tauri dev
```

Esse comando inicia simultaneamente:

* O frontend React
* O backend Rust integrado ao Tauri

---

## Estrutura do Banco de Dados

| Entidade   | Campos principais                                                                     |
| ---------- | ------------------------------------------------------------------------------------- |
| User       | id, username, password_hash, created_at, updated_at                                   |
| Vault      | id, user_id, vault_key_cipher, created_at, updated_at                                 |
| Credential | id, vault_id, name, username?, url?, notes?, password_cipher?, created_at, updated_at |

---

## Fluxo de Uso (CLI)

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

---

## Considerações de Segurança

* Senhas nunca são armazenadas em texto plano
* A chave do cofre permanece cifrada
* Dados sensíveis são removidos da memória quando não são mais necessários, utilizando `zeroize`
* A aplicação funciona totalmente offline

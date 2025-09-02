# 🚛 Transportadora Rust

Sistema de gerenciamento de transportadora desenvolvido em **Rust**, com integração ao **PostgreSQL** e uso do **SQLx** para migrations e consultas.  
Projeto inicial, com estrutura pronta para API, migrations e organização modular.

---

## 📌 Tecnologias Utilizadas
- [Rust](https://www.rust-lang.org/)  
- [SQLx](https://docs.rs/sqlx/latest/sqlx/) (ORM e migrations)  
- [PostgreSQL](https://www.postgresql.org/) (Banco de dados)  
- [Tokio](https://tokio.rs/) (Runtime assíncrona para Rust)  
- [Actix Web](https://actix.rs/) (Framework web para Rust)

---

## ⚙️ Configuração do Projeto

### 1️⃣ Clonar o repositório
```bash
git clone git@github.com:AngelicaRafaela/transportadora-rust.git
cd transportadora-rust
````

### 2️⃣ Configurar variáveis de ambiente

Defina a URL do banco de dados PostgreSQL (já no **MKDB**):

```bash
setx DATABASE_URL "postgresql://dngujnfv:wrcetyfduzvhclrfzfcn@alpha.mkdb.sh:5432/qxfdlhqs"
```

No Linux/MacOS, use:

```bash
export DATABASE_URL="postgresql://dngujnfv:wrcetyfduzvhclrfzfcn@alpha.mkdb.sh:5432/qxfdlhqs"
```

### 3️⃣ Instalar dependências

```bash
cargo build
```

### 4️⃣ Instalar CLI do SQLx

```bash
cargo install sqlx-cli
```

### 5️⃣ Criar migrations

```bash
sqlx migrate add init
sqlx migrate run
```

---

## 🚀 Rodando a aplicação

### Ambiente de desenvolvimento

```bash
cargo run
```

O servidor iniciará (porta será definida no `main.rs` ou em `.env`).

---

## 📂 Estrutura do Projeto

```
transportadora-rust/
│── src/
│   ├── api/           # Endpoints da API (controllers, handlers)
│   ├── models/        # Estruturas de dados (representação de tabelas)
│   ├── routes.rs      # Definições de rotas
│   └── main.rs        # Ponto de entrada da aplicação
│
│── migrations/        # Arquivos de migrations do SQLx
│── Cargo.toml         # Dependências do projeto
│── README.md          # Documentação do projeto
```

---

## 🛠️ Funcionalidades previstas

* Cadastro e gerenciamento de **motoristas**
* Cadastro de **caminhões** e suas informações técnicas
* Controle de **viagens** (origem, destino, distância, status)
* Gestão de **clientes**
* Relatórios básicos

---

## 📄 Licença

Este projeto está sob a licença **MIT** – sinta-se livre para usar, modificar e compartilhar.

---

👩‍💻 Desenvolvido por **[Angélica Rafaela](https://www.linkedin.com/in/angélica-rafaela)**

# Publicando o spec-loop no Crates.io (Cargo Install)

Este guia orienta sobre como publicar o workspace do **spec-loop** no [crates.io](https://crates.io/) para que os usuários possam instalá-lo diretamente via:

```bash
cargo install spec-loop
# Ou (dependendo do nome do pacote publicado):
cargo install specloop-cli
```

---

## 1. Pré-requisitos

1. **Conta no Crates.io**: Crie uma conta ou faça login em [crates.io](https://crates.io/).
2. **Token de API**: Crie um token de API em [crates.io/settings/tokens](https://crates.io/settings/tokens).
3. **Login local**: No terminal, configure sua máquina executando:
   ```bash
   cargo login <seu-token-de-api>
   ```

---

## 2. Ajuste de Dependências do Workspace

O `crates.io` não permite dependências apenas por caminho local (`path = "..."`). Para publicar, todas as dependências internas precisam definir uma versão no `crates.io`.

No `Cargo.toml` raiz (workspace), atualize a seção `[workspace.dependencies]` para incluir as versões correspondentes que serão publicadas:

```toml
# Cargo.toml
[workspace.dependencies]
# ... dependências externas ...

specloop-config = { version = "0.1.0", path = "crates/specloop-config" }
specloop-core = { version = "0.1.0", path = "crates/specloop-core" }
specloop-reporter = { version = "0.1.0", path = "crates/specloop-reporter" }
specloop-safety = { version = "0.1.0", path = "crates/specloop-safety" }
specloop-schemas = { version = "0.1.0", path = "crates/specloop-schemas" }
specloop-templates = { version = "0.1.0", path = "crates/specloop-templates" }
```

> [!IMPORTANT]
> A licença atual no `Cargo.toml` (`LicenseRef-PolyForm-Noncommercial-1.0.0`) pode precisar ser configurada ou validada de acordo com as regras de publicação do Crates.io. Caso contrário, adicione campos apropriados ou registre-se sob termos compatíveis.

---

## 3. Ordem de Publicação (Topológica)

Como as crates dependem umas das outras, a publicação deve ocorrer das folhas (sem dependências internas) até o topo (a CLI). Execute a publicação no terminal na seguinte ordem:

```bash
# 1. Crates base (sem dependências internas do workspace)
cargo publish -p specloop-schemas
cargo publish -p specloop-templates

# 2. Crates intermediárias
cargo publish -p specloop-reporter
cargo publish -p specloop-safety
cargo publish -p specloop-config

# 3. Núcleo (Core)
cargo publish -p specloop-core

# 4. Interface de Linha de Comando (CLI)
cargo publish -p specloop-cli
```

*(Dica: aguarde alguns segundos entre cada comando para o crates.io indexar a versão recém-publicada).*

---

## 4. Renomeando a CLI para `spec-loop` (Opcional)

Atualmente, o pacote CLI se chama `specloop-cli` em `crates/specloop-cli/Cargo.toml`, mas expõe o binário `specloop`.

Se você deseja que os usuários instalem com o nome exato do projeto (`cargo install spec-loop`), altere o nome do pacote na definição da crate CLI:

```toml
# crates/specloop-cli/Cargo.toml
[package]
name = "spec-loop" # Alterado de specloop-cli para spec-loop
```

Se fizer isso, o comando de instalação será:
```bash
cargo install spec-loop
```

---

## 5. Como o Usuário Instala e Executa

Uma vez publicado, qualquer usuário com a toolchain do Rust instalada poderá rodar:

```bash
cargo install spec-loop
```

E o binário estará disponível no terminal:

```bash
specloop --help
```

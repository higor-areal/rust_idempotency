# 🔁 Rust Idempotency API

Uma API construída com **Rust**, **Axum** e **Tokio** para estudar e implementar **idempotência em APIs HTTP**.

Este projeto simula um sistema de pagamentos/processamento de operações críticas, garantindo que uma mesma request não seja executada múltiplas vezes mesmo quando enviada repetidamente.

---

# 📚 O que é idempotência?

Idempotência é a capacidade de executar a mesma operação várias vezes sem alterar o resultado final após a primeira execução.

Exemplo simples:

```txt
clicar 1 vez em "pagar"
↓
pagamento processado

clicar 20 vezes rapidamente
↓
continua sendo apenas 1 pagamento
```

---

## ❌ Problema sem idempotência

Imagine um cliente fazendo uma compra:

```txt
POST /payment
```

A internet oscila.

O frontend não sabe se a request funcionou.

Então ele tenta novamente.

Sem idempotência:

```txt
1 request
→ cobra 100 reais

2 request
→ cobra mais 100 reais
```

Resultado:

💀 cobrança duplicada.

---

## ✅ Solução com idempotência

O cliente envia um identificador único:

```txt
Idempotency-Key: abc123
```

A API:

- verifica se aquela chave já foi usada
- se já existir:
  - retorna mesma resposta anterior
- se não existir:
  - processa normalmente
  - salva resultado

Fluxo:

```txt
request
→ verifica chave
→ já existe?
    sim → retorna resposta salva
    não → processa e salva
```

---

## 🎯 Objetivo do projeto

Este projeto foi criado para praticar:

- idempotência
- middleware
- concorrência
- shared state
- HashMap
- Mutex assíncrono
- arquitetura modular
- APIs resilientes

---

# ✨ Funcionalidades

- 🔁 Idempotência via header
- 🧠 Cache de requests processadas
- ⚡ Prevenção de duplicidade
- 🛡️ Middleware para interceptação
- 📦 Shared state
- 📋 Consulta de requests processadas

---

# 🛠️ Stack utilizada

- Rust 🦀
- Axum
- Tokio
- Serde

---

# 📁 Estrutura do projeto

```txt
src/
├── handlers/
│   ├── mod.rs
│   └── payment_handler.rs
│
├── middleware/
│   ├── mod.rs
│   └── idempotency_middleware.rs
│
├── models/
│   ├── mod.rs
│   ├── payment.rs
│   └── processed_request.rs
│
├── responses/
│   ├── mod.rs
│   └── response.rs
│
├── state/
│   ├── mod.rs
│   └── app_state.rs
│
└── main.rs
```

---

# ⚙️ Como funciona

O cliente envia:

```txt
Idempotency-Key: pagamento123
```

A API:

1. verifica se chave já existe
2. se existir:
   - retorna resposta salva
3. se não existir:
   - processa pagamento
   - salva resposta
   - marca chave como usada

---

# 🧠 Estado da aplicação

A aplicação mantém requests processadas em memória.

Exemplo:

```rust
HashMap<String, ProcessedRequest>
```

Onde:

```txt
chave idempotente
↓
resultado da request
```

---

# 📌 Endpoints

## GET /

Health check.

Resposta:

```json
{
  "message": "API rodando"
}
```

---

## POST /payment

Simula processamento de pagamento.

Headers:

```txt
Idempotency-Key: pagamento123
```

Body:

```json
{
  "client": "esau",
  "amount": 100.0
}
```

---

## Primeira request

Resposta:

```json
{
  "status_code": 201,
  "message": "Pagamento processado"
}
```

Fluxo:

```txt
request
→ chave não existe
→ processa
→ salva
```

---

## Segunda request com mesma chave

Resposta:

```json
{
  "status_code": 200,
  "message": "Request reutilizada"
}
```

Fluxo:

```txt
request
→ chave encontrada
→ retorna resposta salva
```

---

## GET /requests

Lista requests processadas.

Resposta:

```json
[
  {
    "idempotency_key": "pagamento123",
    "status": "processed"
  }
]
```

---

# 🔐 Middleware de idempotência

O middleware intercepta requests antes dos handlers.

Fluxo:

```txt
request
↓
middleware
↓
extrai Idempotency-Key
↓
verifica HashMap
↓
já existe?
↓
sim → retorna resposta salva
não → continua request
```

Trecho simplificado:

```rust
if exists {
    return saved_response;
}

next.run(request).await
```

---

# 🧵 Concorrência

A aplicação utiliza:

```rust
tokio::sync::Mutex
```

junto com:

```rust
Arc
```

para permitir:

- múltiplos donos
- acesso seguro
- mutabilidade compartilhada

---

# ▶️ Executando localmente

Clone:

```bash
git clone https://github.com/higor-areal/rust-idempotency-api.git
```

Entre no projeto:

```bash
cd rust-idempotency-api
```

Execute:

```bash
cargo run
```

Servidor:

```txt
http://localhost:3000
```

---

# 🧪 Testando

Primeira request:

```bash
curl -X POST http://localhost:3000/payment \
-H "Content-Type: application/json" \
-H "Idempotency-Key: pagamento123" \
-d '{"client":"esau","amount":100.0}'
```

Repita novamente:

```bash
curl -X POST http://localhost:3000/payment \
-H "Content-Type: application/json" \
-H "Idempotency-Key: pagamento123" \
-d '{"client":"esau","amount":100.0}'
```

Resultado esperado:

- primeira request processa ✅
- segunda reutiliza resposta ✅

---

# 🎯 Objetivos de aprendizado

Este projeto explora:

- middleware
- idempotência
- concorrência
- async Mutex
- HashMap
- shared state
- arquitetura modular
- prevenção de duplicidade

---

# 🚀 Melhorias futuras

- Redis para persistência
- expiração automática de chaves
- banco de dados
- retry distribuído
- lock distribuído
- persistência real de requests
- logs estruturados

---

# 👨‍💻 Autor

GitHub:  
https://github.com/higor-areal
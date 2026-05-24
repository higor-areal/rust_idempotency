# 🔁 Rust Idempotency API

Uma API robusta desenvolvida com **Rust**, **Axum** e **Tokio** focada na implementação de **idempotência** para operações críticas, como processamento de pagamentos. Este projeto demonstra como garantir a consistência de dados e prevenir execuções duplicadas em sistemas distribuídos.

---

## 📚 O que é Idempotência?

A **idempotência** é a propriedade de certas operações em matemática e ciência da computação que podem ser aplicadas várias vezes sem alterar o resultado além da aplicação inicial. No contexto de APIs HTTP, uma requisição idempotente é aquela em que o efeito colateral de realizar a mesma requisição múltiplas vezes é o mesmo que realizá-la apenas uma vez.

> "Se um cliente envia a mesma instrução de pagamento dez vezes devido a uma falha de rede, o sistema deve processar a cobrança apenas uma vez."

Neste projeto, a idempotência é implementada através do header `Idempotency-Key` e da validação do hash do payload da requisição. Isso garante que, mesmo que uma requisição seja enviada várias vezes, a operação subjacente (como um pagamento) seja executada apenas uma vez, e as respostas subsequentes retornem o resultado da primeira execução.

---

## 🛠️ Tecnologias Utilizadas

O projeto utiliza o ecossistema moderno de Rust para alta performance e segurança:

- **Rust 🦀**: Linguagem base focada em segurança de memória e performance.
- **Axum**: Framework web ergonômico e modular construído sobre a stack `tower` e `hyper`.
- **Tokio**: Runtime assíncrono para lidar com concorrência de forma eficiente.
- **Serde**: Framework para serialização e desserialização de dados (JSON).
- **SHA-2**: Utilizado para gerar hashes únicos dos payloads das requisições, garantindo a integridade dos dados.

---

## ⚙️ Como a Idempotência foi Implementada

A implementação baseia-se em um **Middleware de Interceptação** que atua antes do processamento final da rota, garantindo que as requisições sejam verificadas quanto à sua idempotência.

### 1. Interceptação via Middleware
O arquivo `src/middleware/idempotency_middleware.rs` contém a lógica principal. Ele intercepta todas as requisições para a rota `/payment`. O middleware extrai o header `Idempotency-Key` da requisição. Se a chave estiver ausente, a requisição é rejeitada com um `400 Bad Request`.

```rust
// Trecho de src/middleware/idempotency_middleware.rs
pub async fn idempotency_middleware(
    State(state): State<Arc<Mutex<AppState>>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let key = match get_idempotency_key(request.headers()) {
        Some(key) => key,
        None => return bad_request("Idempotency-Key ausente")
    };
    // ... (lógica de processamento do body e hash)
}
```

### 2. Validação de Integridade (Payload Hash)
Para garantir que a mesma `Idempotency-Key` não seja reutilizada com payloads diferentes, o sistema gera um hash do corpo da requisição. Este hash é criado pelo método `hash()` na estrutura `Payment` (`src/models/payment.rs`). Se uma `Idempotency-Key` já existir no estado da aplicação, mas o hash do payload da requisição atual for diferente do hash armazenado, a API retorna um `409 Conflict`, protegendo contra o reuso indevido de chaves para operações distintas.

```rust
// Trecho de src/models/payment.rs
impl Payment{
    pub fn hash(&mut self) -> Option<String>{
        let json = match serde_json::to_string(&self) {
            Ok(t) => t,
            Err(_) => return None,
        };
        let mut hasher = Sha256::new();
        hasher.update(json);
        let result = hasher.finalize();
        Some(format!("{:x}", result))
    }
}
```

### 3. Gerenciamento de Estado
O estado da aplicação (`AppState` em `src/state/app_state.rs`) é compartilhado entre as threads usando `Arc<Mutex<AppState>>`. Isso permite que o middleware acesse e modifique de forma segura um `HashMap` que armazena as `ProcessedRequest` (requisições já processadas ou em processamento), mapeando a `Idempotency-Key` para o resultado da requisição.

```rust
// Trecho de src/state/app_state.rs
pub struct AppState{
    pub requests: HashMap<String,ProcessedRequest>
}

// Trecho de src/models/processed_request.rs
pub struct ProcessedRequest {
    pub payload_hash: String,
    pub response: Option<String>,
    pub status_code: Option<u16>,
    pub status: RequestStatus,
}
```

Quando uma requisição com uma `Idempotency-Key` é recebida:
- Se a chave já existe no `HashMap` e o `payload_hash` corresponde, a resposta armazenada é retornada imediatamente.
- Se a chave não existe, a requisição é processada, seu resultado é armazenado no `HashMap` junto com o `payload_hash`, e então a resposta é enviada.

---

## 📌 Endpoints da API

### `POST /payment`
Simula o processamento de um pagamento. Este endpoint é protegido pelo middleware de idempotência.

- **Headers Obrigatórios**:
  - `Idempotency-Key`: Uma string única (UUID, por exemplo) que identifica a requisição.

- **Body da Requisição**:
```json
{
  "client": "nome_do_cliente",
  "amount": 100.0
}
```

- **Comportamento Esperado**:
  - **Primeira Requisição**: O pagamento é processado, o resultado é armazenado no estado da aplicação, e uma resposta `201 Created` é retornada. Exemplo de resposta:
    ```json
    {
      "payload_hash": "hash_do_payload",
      "response": "Pagamento processado",
      "status_code": 201,
      "status": "Completed"
    }
    ```
  - **Requisições Subsequentes (mesma `Idempotency-Key` e `payload_hash`)**: A API retorna a resposta armazenada da primeira execução com um status `200 OK`, sem reprocessar o pagamento. Exemplo de resposta:
    ```json
    {
      "payload_hash": "hash_do_payload",
      "response": "Pagamento processado",
      "status_code": 201,
      "status": "Completed"
    }
    ```
  - **Requisições com `Idempotency-Key` existente, mas `payload_hash` diferente**: A API retorna um `409 Conflict` para indicar que a chave está sendo usada de forma inconsistente.

### `GET /process`
Retorna um mapa de todas as requisições que foram processadas ou que estão em processamento pelo sistema, indexadas pela `Idempotency-Key`.

- **Exemplo de Resposta**:
```json
{
  "chave-exemplo-123": {
    "payload_hash": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2",
    "response": "Pagamento processado",
    "status_code": 201,
    "status": "Completed"
  },
  "outra-chave-456": {
    "payload_hash": "b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3",
    "response": null,
    "status_code": null,
    "status": "Processing"
  }
}
```

### `GET /`
Um endpoint simples para verificar a saúde da API.

- **Resposta**:
```
Root API
```

---

## 📁 Estrutura de Pastas

```txt
src/
├── handlers/      # Contém a lógica para as rotas da API, como `payment_handler.rs`.
├── middleware/    # Define o middleware de idempotência em `idempotency_middleware.rs`.
├── models/        # Estruturas de dados como `Payment` e `ProcessedRequest`.
├── responses/     # Funções auxiliares para padronizar as respostas da API.
├── state/         # Gerencia o estado compartilhado da aplicação, `AppState`.
└── main.rs        # Ponto de entrada da aplicação, onde as rotas são configuradas.
```

---

## 🚀 Como Executar Localmente

Para rodar este projeto em sua máquina local, siga os passos abaixo:

1.  **Pré-requisitos**: Certifique-se de ter o [Rust toolchain](https://www.rust-lang.org/tools/install) instalado.

2.  **Clonar o Repositório**:
    ```bash
    git clone https://github.com/higor-areal/rust-idempotency-api.git 
    cd rust-idempotency-api
    ```

3.  **Executar a Aplicação**:
    ```bash
    cargo run
    ```

    A API estará disponível em `http://localhost:3000`.

---

## 🧪 Testando a Idempotência

Você pode testar a funcionalidade de idempotência usando `curl`:

1.  **Primeira Requisição (processamento inicial)**:
    ```bash
    curl -X POST http://localhost:3000/payment \
    -H "Content-Type: application/json" \
    -H "Idempotency-Key: pagamento123" \
    -d '{"client":"esau","amount":100.0}'
    ```
    Você deve receber uma resposta indicando que o pagamento foi processado (`201 Created`).

2.  **Segunda Requisição (reutilização da resposta)**:
    ```bash
    curl -X POST http://localhost:3000/payment \
    -H "Content-Type: application/json" \
    -H "Idempotency-Key: pagamento123" \
    -d '{"client":"esau","amount":100.0}'
    ```
    Desta vez, a API deve retornar a mesma resposta da primeira requisição, mas com um status `200 OK`, indicando que a operação não foi reprocessada.

3.  **Verificar Requisições Processadas**:
    ```bash
    curl http://localhost:3000/process
    ```
    Isso exibirá o estado atual das requisições processadas, incluindo a `pagamento123`.

---

## 👨‍💻 Autor

Projeto desenvolvido para estudo de arquiteturas resilientes e concorrência em Rust por HIGOR ESAÚ.

# Rust API Send Message Telegram

API que recebe uma requisição HTTP POST com um email e uma mensagem, formata esses dados e envia para um chat do Telegram através da API do Telegram Bot.

## Dependências

- `actix-web`: Usado para criar o servidor HTTP e a rota POST /send_message
- `reqwest`: Usado para fazer a requisição HTTP para a API do Telegram
- `tokio`: Necessário para executar operações assíncronas como requisições HTTP
- `serde`: Usado para converter o JSON da requisição em uma struct Rust
- `serde_json`: Usado para criar o JSON que será enviado para a API do Telegram
- `dotenv`: Usado para carregar as variáveis de ambiente TELEGRAM_TOKEN e CHAT_ID 
use actix_web::{web, App, HttpResponse, HttpServer};
use reqwest::Client;
use serde::Deserialize; 
use std::env;

#[derive(Deserialize)]
struct TelegramRequest {
    email: String,
    message: String,
}

async fn send_telegram_message(item: web::Json<TelegramRequest>) -> HttpResponse {
    let chat_id_str = match env::var("CHAT_ID") {
        Ok(val) => val,
        Err(_) => return HttpResponse::InternalServerError().body("CHAT_ID não configurado"),
    };
    let chat_id: i64 = match chat_id_str.parse() {
        Ok(val) => val,
        Err(_) => return HttpResponse::InternalServerError().body("Formato de CHAT_ID inválido"),
    };
    let telegram_token = match env::var("TELEGRAM_TOKEN") {
        Ok(val) => val,
        Err(_) => return HttpResponse::InternalServerError().body("TELEGRAM_TOKEN não configurado"),
    };

    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        telegram_token
    );

    let client = Client::new();

    let full_message = format!("Email: {}\nMensagem: {}", item.email, item.message);

    match client
        .post(&url)
        .json(&serde_json::json!({
            "chat_id": chat_id,
            "text": full_message
        }))
        .send()
        .await
    {
        Ok(res) => {
            let status = res.status(); // Captura o status antes de consumir 'res'
            if status.is_success() {
                HttpResponse::Ok().body("Mensagem enviada com sucesso para o Telegram!")
            } else {
                let error_text = res.text().await.unwrap_or_else(|_| "Erro desconhecido".to_string());
                HttpResponse::InternalServerError().body(format!("Erro ao enviar mensagem para o Telegram: Status {} - {}", status, error_text))
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Erro de rede ao enviar mensagem: {}", e)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new().service(
            web::resource("/send_message")
                .route(web::post().to(send_telegram_message)),
        )
    })
    .bind(("0.0.0.0", 7000))?
    .run()
    .await?;

    Ok(())
}

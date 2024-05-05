use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::env;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::process::Command;

#[derive(Serialize, Deserialize)]
struct FileHashResponse {
    file_name: String,
    hash: String,
}

async fn handle_file_upload(mut payload: web::Payload) -> Result<impl Responder> {
    let mut file = File::create("uploaded_file").await.map_err(|e| {
        eprintln!("Failed to create file: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    while let Some(chunk) = payload.next().await {
        let data = chunk.map_err(|e| {
            eprintln!("Error processing chunk: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;
        file.write_all(&data).await.map_err(|e| {
            eprintln!("Failed to write to file: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    }

    let file_hash = calculate_file_hash("uploaded_file").await;
    invoke_blockchain_integration("uploaded_file", &file_hash).await;

    Ok(web::Json(FileHashResponse {
        file_name: "uploaded_file".to_string(),
        hash: file_hash,
    }))
}

async fn calculate_file_hash(file_name: &str) -> String {
    let mut file = match File::open(file_name).await {
        Ok(file) => file,
        Err(_) => return String::from("Error"),
    };
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 1024];

    loop {
        let read_bytes = match file.read(&mut buffer).await {
            Ok(count) => count,
            Err(_) => return String::from("Error"),
        };
        if read_bytes == 0 {
            break;
        }
        hasher.update(&buffer[..read_bytes]);
    }

    format!("{:x}", hasher.finalize())
}

async fn invoke_blockchain_integration(file_name: &str, file_hash: &str) {
    let block_endpoint = env::var("BLOCKCHAIN_ENDPOINT").unwrap_or_else(|_| "http://localhost:5000".to_string());
    let python_executable = env::var("PYTHON_COMMAND").unwrap_or_else(|_| "python".to_string());
    let integration_script_path = env::var("PYTHON_SCRIPT_PATH").unwrap_or_else(|_| "blockchain_interface.py".to_string());

    let execution_status = Command::new(python_executable)
        .arg(integration_script_path)
        .arg(&block_endpoint)
        .arg(file_name)
        .arg(file_hash)
        .status()
        .await
        .expect("Failed to execute command");

    println!("Python script execution status: {:?}", execution_status);
}

async fn delete_uploaded_file() -> impl Responder {
    match tokio::fs::remove_file("uploaded_file").await {
        Ok(_) => HttpResponse::Ok().body("File successfully deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete file"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/upload").route(web::post().to(handle_file_upload)))
            .service(web::resource("/delete").route(web::delete().to(delete_uploaded_file)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
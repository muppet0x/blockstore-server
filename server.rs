use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use std::env;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::process::Command;

#[derive(Serialize, Deserialize)]
struct UploadedFileHash {
    file_name: String,
    hash: String,
}

async fn upload_file_handler(mut payload: web::Payload) -> Result<impl Responder> {
    let mut uploaded_file = File::create("uploaded_file").await.map_err(|e| {
        eprintln!("Failed to create file: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    while let Some(chunk) = payload.next().await {
        let data_chunk = chunk.map_err(|e| {
            eprintln!("Error processing chunk: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;
        uploaded_file.write_all(&data_chunk).await.map_err(|e| {
            eprintln!("Failed to write to file: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    }

    let hash_for_file = generate_file_hash("uploaded_file").await;
    trigger_blockchain_integration("uploaded_file", &hash_for_file).await;

    Ok(web::Json(UploadedFileHash {
        file_name: "uploaded_file".to_string(),
        hash: hash_for_file,
    }))
}

async fn generate_file_hash(file_name: &str) -> String {
    let mut file_to_hash = match File::open(file_name).await {
        Ok(file) => file,
        Err(_) => return String::from("Error"),
    };
    let mut hasher = Sha256::new();
    let mut read_buffer = vec![0; 1024];

    loop {
        let bytes_read = match file_to_hash.read(&mut read_buffer).await {
            Ok(count) => count,
            Err(_) => return String::from("Error"),
        };
        if bytes_read == 0 {
            break;
        }
        hasher.update(&read_buffer[..bytes_read]);
    }

    format!("{:x}", hasher.finalize())
}

async fn trigger_blockchain_integration(file_name: &str, file_hash: &str) {
    let blockchain_endpoint = env::var("BLOCKCHAIN_ENDPOINT").unwrap_or_else(|_| "http://localhost:5000".to_string());
    let python_interpreter_path = env::var("PYTHON_COMMAND").unwrap_or_else(|_| "python".to_string());
    let blockchain_script_path = env::var("PYTHON_SCRIPT_PATH").unwrap_or_else(|_| "blockchain_interface.py".to_string());

    let script_run_status = Command::new(python_interpreter_path)
        .arg(blockchain_script_path)
        .arg(&blockchain_endpoint)
        .arg(file_name)
        .arg(file_hash)
        .status()
        .await
        .expect("Failed to execute command");

    println!("Blockchain script run status: {:?}", script_run_status);
}

async fn delete_file_endpoint() -> impl Responder {
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
            .service(web::resource("/upload").route(web::post().to(upload_file_handler)))
            .service(web::resource("/delete").route(web::delete().to(delete_file_endpoint)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
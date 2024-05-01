use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io::Write;
use futures::StreamExt;
use std::{env, fs::File};
use std::process::Command;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Serialize, Deserialize)]
struct FileHashResponse {
    file_name: String,
    hash: String,
}

async fn upload_file(mut payload: web::Payload) -> impl Responder {
    let mut file = File::create("uploaded_file").unwrap();
    while let Some(chunk) = payload.next().await {
        let data = chunk.unwrap();
        file.write_all(&data).unwrap();
    }
    let file_hash = compute_file_hash("uploaded_file").await;
    invoke_python_blockchain("uploaded_file", &file_hash).await;
    web::Json(FileHashResponse {
        file_name: "uploaded_file".to_string(),
        hash: file_hash,
    })
}

async fn compute_file_hash(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let count = file.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    format!("{:x}", hasher.finalize())
}

async fn invoke_python_blockchain(file_name: &str, file_hash: &str) {
    let blockchain_endpoint = env::var("BLOCKCHAIN_ENDPOINT").unwrap_or_else(|_| "http://localhost:5000".to_string());

    let python_command = env::var("PYTHON_COMMAND").unwrap_or_else(|_| "python".to_string());
    let script_path = env::var("PYTHON_SCRIPT_PATH").unwrap_or_else(|_| "blockchain_interface.py".to_string());

    let output = Command::new(python_command)
        .arg(script_path)
        .arg(blockchain_endpoint)
        .arg(file_name)
        .arg(file_hash)
        .output()
        .expect("Failed to execute command");

    println!("Python script output: {:?}", output);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/upload").route(web::post().to(upload_file)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
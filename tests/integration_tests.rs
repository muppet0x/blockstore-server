use std::process::Command;
use std::env;
use dotenv::dotenv;
use std::io::ErrorKind;

#[tokio::test]
async fn test_file_upload_and_hash_storage() {
    dotenv().ok();
    let server_url = match env::var("SERVER_URL") {
        Ok(val) => val,
        Err(_) => panic!("SERVER_URL must be set in .env"),
    };

    let python_script_path = match env::var("PYTHON_SCRIPT_PATH") {
        Ok(val) => val,
        Err(_) => panic!("PYTHON_SCRIPT_PATH must be set in .env"),
    };

    let blockchain_address = match env::var("BLOCKCHAIN_ADDRESS") {
        Ok(val) => val,
        Err(_) => panic!("BLOCKCHAIN_ADDRESS must be set in .env"),
    };

    let upload_response = simulate_file_upload(&server_url).await;
    assert!(upload_response.is_success, "File upload failed");

    let file_hash = extract_hash_from_upload_response(upload_response);

    // Improved error handling for Python script execution
    match Command::new("python")
        .arg(&python_script_path)
        .arg("--hash")
        .arg(&file_hash)
        .output() {
            Ok(output) => {
                if !output.status.success() {
                    let error_message = format!("Python script failed to verify hash. Error: {:?}", 
                                                String::from_utf8_lossy(&output.stderr));
                    panic!("{}", error_message);
                }
            },
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => panic!("Python executable not found in path"),
                    _ => panic!("Failed to run Python script for hash verification: {:?}", e)
                }
            },
    };

    let blockchain_verification = verify_hash_on_blockchain(&blockchain_address, &file_hash).await;
    assert!(blockchain_verification, "Hash not found on the blockchain");
}

async fn simulate_file_upload(server_url: &str) -> UploadResponse {
    // Here you should implement actual file upload logic
    // This is just a dummy response for now
    UploadResponse { is_success: true }
}

fn extract_hash_from_upload_response(response: UploadResponse) -> String {
    // You should implement the actual logic to extract hash based on your response structure
    "dummy_hash".to_string()
}

async fn verify_hash_on_blockchain(blockchain_address: &str, file_hash: &str) -> bool {
    // Here you should implement the actual logic to verify the hash on the blockchain
    // This is just a dummy implementation for now
    true
}

struct UploadResponse {
    is_success: bool,
}
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::process::Command;
use std::env;
use dotenv::dotenv;
use std::io::ErrorKind;

static BLOCKCHAIN_VERIFICATION_CACHE: Lazy<Mutex<HashMap<String, bool>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[tokio::test]
async fn test_file_upload_and_blockchain_hash_verification() {
    dotenv().ok();
    let serverEndpoint = env::var("SERVER_URL").expect("SERVER_URL must be set in .env");
    let pathToPythonScript = env::var("PYTHON_SCRIPT_PATH").expect("PYTHON_SCRIPT_PATH must be set in .env");
    let blockchainWalletAddress = env::var("BLOCKCHAIN_ADDRESS").expect("BLOCKCHAIN_ADDRESS must be set in .env");

    let uploadResult = simulate_upload_process(&serverEndpoint).await;
    assert!(uploadResult.successful, "File upload failed");

    let hashOfUploadedFile = extract_hash_from_response(uploadResult);

    match Command::new("python")
        .arg(&pathToPythonScript)
        .arg("--hash")
        .arg(&hashOfUploadedFile)
        .output() {
            Ok(output) => {
                if !output.status.success() {
                    let errorMessage = format!("Python script failed to verify file hash. Error: {:?}", 
                                                String::from_utf8_lossy(&output.stderr));
                    panic!("{}", errorMessage);
                }
            },
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => panic!("Python executable not found in path"),
                    _ => panic!("Failed to execute Python script for hash verification: {:?}", error)
                }
            },
    };

    let isHashVerifiedOnBlockchain = confirm_hash_on_blockchain_cached(&blockchainWalletAddress, &hashOfUploadedFile).await;
    assert!(isHashVerifiedOnBlockchain, "Hash not verified on the blockchain");
}

async fn simulate_upload_process(serverEndpoint: &str) -> UploadOutcome {
    UploadOutcome { successful: true }
}

fn extract_hash_from_response(response: UploadOutcome) -> String {
    "example_hash".to_string()
}

async fn confirm_hash_on_blockchain(blockchainAddress: &str, fileHash: &str) -> bool {
    true
}

async fn confirm_hash_on_blockchain_cached(blockchainAddress: &str, fileHash: &str) -> bool {
    let cache_key = format!("{}:{}", blockchainAddress, fileHash);
    let mut cache = BLOCKCHAIN_VERIFICATION_CACHE.lock().unwrap();
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }
    let result = confirm_hash_on_blockchain(blockchainAddress, fileHash).await;
    cache.insert(cache_key, result);
    result
}

struct UploadOutcome {
    successful: bool,
}
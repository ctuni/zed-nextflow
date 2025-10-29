use std::env;
use std::path::PathBuf;

use reqwest::header::USER_AGENT;
use reqwest::Client;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    println!("Starting REAL JAR downloader for nextflow-extension");

    let base_dir = env::var("ZED_EXTENSION_DIR").unwrap_or_else(|_| {
        println!("ZED_EXTENSION_DIR not set, defaulting to '.'");
        ".".to_string()
    });

    let jar_path = PathBuf::from(&base_dir).join("language-server-all.jar");

    println!("JAR will be stored at: {:?}", jar_path);

    if jar_path.exists() {
        println!("Found existing JAR at {:?}", jar_path);
        return;
    }

    let simulated_version = "v25.04.3";
    let download_url = format!(
        "https://github.com/nextflow-io/language-server/releases/download/{}/language-server-all.jar",
        simulated_version
    );

    println!("Downloading from: {}", download_url);

    let client = Client::new();
    let response = client
        .get(&download_url)
        .header(USER_AGENT, "zed-nextflow-extension-test")
        .send()
        .await;

    let response = match response {
        Ok(resp) if resp.status().is_success() => resp,
        Ok(resp) => {
            eprintln!("Failed: Got status code {}", resp.status());
            return;
        }
        Err(e) => {
            eprintln!("Request error: {e}");
            return;
        }
    };

    let body = match response.bytes().await {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Failed to read response body: {}", e);
            return;
        }
    };

    // Try to create the file asynchronously
    match fs::File::create(&jar_path).await {
        Ok(mut file) => {
            // Write the bytes asynchronously
            match file.write_all(&body).await {
                Ok(_) => println!(
                    "Successfully downloaded {} bytes to {:?}",
                    body.len(),
                    jar_path
                ),
                Err(e) => eprintln!("Failed to write to file: {e}"),
            }
        }
        Err(e) => {
            eprintln!("Failed to create JAR file at {:?}: {e}", jar_path);
        }
    }

    println!("Finished real test run.");
}

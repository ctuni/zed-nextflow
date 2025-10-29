use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;

fn main() {
    println!("\u{1f680} Starting REAL JAR downloader for nextflow-extension");

    let base_dir = env::var("ZED_EXTENSION_DIR").unwrap_or_else(|_| {
        println!("⚠️  ZED_EXTENSION_DIR not set, defaulting to '.'");
        ".".to_string()
    });

    let jar_path = PathBuf::from(&base_dir).join("language-server-all.jar");

    println!("📦 JAR will be stored at: {:?}", jar_path);

    if jar_path.exists() {
        println!("✅ Found existing JAR at {:?}", jar_path);
        return;
    }

    let simulated_version = "v25.04.3";
    let download_url = format!(
        "https://github.com/nextflow-io/language-server/releases/download/{}/language-server-all.jar",
        simulated_version
    );

    println!("🌐 Downloading from: {}", download_url);

    let client = Client::new();
    let response = client
        .get(&download_url)
        .header(USER_AGENT, "zed-nextflow-extension-test")
        .send();

    let mut response = match response {
        Ok(resp) if resp.status().is_success() => resp,
        Ok(resp) => {
            eprintln!("❌ Failed: Got status code {}", resp.status());
            return;
        }
        Err(e) => {
            eprintln!("❌ Request error: {e}");
            return;
        }
    };

    // Try to create the file
    match fs::File::create(&jar_path) {
        Ok(mut file) => {
            match response.copy_to(&mut file) {
                Ok(bytes) => println!("✅ Successfully downloaded {} bytes to {:?}", bytes, jar_path),
                Err(e) => eprintln!("❌ Failed to write to file: {e}"),
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create JAR file at {:?}: {e}", jar_path);
        }
    }

    println!("🏁 Finished real test run.");
}

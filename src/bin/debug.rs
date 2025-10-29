use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("🚀 Starting standalone debug for nextflow-extension");

    // 1. Resolve where the JAR should go
    let base_dir = env::var("ZED_EXTENSION_DIR").unwrap_or_else(|_| {
        println!("⚠️ ZED_EXTENSION_DIR not set, defaulting to '.'");
        ".".to_string()
    });

    let jar_path = PathBuf::from(base_dir).join("language-server-all.jar");

    println!("📦 Expected JAR path: {:?}", jar_path);

    // 2. Check if file already exists
    if jar_path.is_file() {
        println!("✅ Found JAR at {:?}", jar_path);
    } else {
        println!("❌ JAR not found. Will simulate download logic.");

        // 3. Simulate a GitHub release version (normally from API)
        let simulated_version = "v25.04.3";
        let download_url = format!(
            "https://github.com/nextflow-io/language-server/releases/download/{}/language-server-all.jar",
            simulated_version
        );

        println!("🌐 Would download from: {}", download_url);

        // 4. Try to create the file to test permissions
        match fs::write(&jar_path, b"dummy JAR contents") {
            Ok(_) => println!("✅ Successfully wrote dummy JAR file at {:?}", jar_path),
            Err(e) => println!("❌ Failed to write JAR file: {e}"),
        }

        // 5. Clean up dummy file
        let _ = fs::remove_file(&jar_path);
    }

    println!("🏁 Finished test run.");
}

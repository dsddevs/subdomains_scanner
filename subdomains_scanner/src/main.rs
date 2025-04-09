use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::{env, process};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage {} <domain>", args[0]);
        process::exit(1)
    }

    let domain = &args[1];
    let response = get_response(domain).await?;
    create_text_file(&response.as_bytes())?;
    Ok(())
}

async fn get_response(domain: &str) -> Result<String, Box<dyn Error>> {
    let url_path = format!("https://crt.sh/?q=%25.{}&output=json", domain);
    let response = reqwest::get(&url_path).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;
    let pretty = serde_json::to_string_pretty(&json)?;
    Ok(pretty)
}

fn create_text_file(response: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("subdomains.json")?;
    file.write_all(response)?;
    Ok(())
}

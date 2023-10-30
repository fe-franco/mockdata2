use std::io::Write;

use rand::{self, Rng};
use reqwest::Client;

pub(crate) fn random_cpf() -> String {
    // returns a valid CPF
    let mut rng = rand::thread_rng();
    let cpf: String = format!(
        "{:03}{:03}{:03}{:02}",
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..99)
    );
    cpf
}

pub(crate) fn random_rg() -> String {
    // returns a valid RG
    let mut rng = rand::thread_rng();
    let rg: String = format!(
        "{:03}{:03}{:03}{:02}",
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..99)
    );
    rg
}

pub(crate) fn random_cnpj() -> String {
    // returns a valid CNPJ
    let mut rng = rand::thread_rng();
    let cnpj: String = format!(
        "{:02}{:03}{:03}{:04}{:02}",
        rng.gen_range(0..99),
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..9999),
        rng.gen_range(0..99)
    );
    cnpj
}

pub(crate) fn random_br_phone() -> u32 {
    let mut rng = rand::thread_rng();

    // Generate a number between 9000_0000 and 9999_9999
    let phone_number: u32 = rng.gen_range(9_000_0000..=9_999_9999);

    phone_number
}

pub(crate) fn random_cep() -> String {
    // returns a valid BR CEP
    let mut rng = rand::thread_rng();
    let cep: String = format!(
        "{:05}-{:03}",
        rng.gen_range(0..99999),
        rng.gen_range(0..999)
    );
    cep
}

pub(crate) async fn fetch_data<T: for<'a> serde::de::Deserialize<'a>>(
    client: &Client,
    url: &str,
) -> Result<T, anyhow::Error> {
    let body = client.get(url).send().await?.text().await?;
    Ok(serde_json::from_str(&body)?)
}

pub(crate) async fn fetch_with_exponential_backoff<T: for<'a> serde::de::Deserialize<'a>>(
    client: &Client,
    url: &str,
) -> Result<T, anyhow::Error> {
    let mut delay = 5; // Start with a 5-second delay

    loop {
        match fetch_data::<T>(&client, &url).await {
            Ok(result) => return Ok(result),
            Err(error) => {
                println!("Error: {}", url);
                if delay > 20 {
                    println!("This is taking too long, skipping...");
                    println!(
                        "last returned body: {}",
                        client.get(url).send().await?.text().await?
                    );
                    println!("last error: {}", error);
                    // skip this url
                    return Err(anyhow::Error::msg(error));
                }

                for remaining in (1..=delay).rev() {
                    print!("\rRetrying in {} seconds... ", remaining);
                    std::io::stdout().flush().unwrap();
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                print!("\r"); // Clear the line after the countdown

                // Double the delay for the next iteration
                delay *= 2;
            }
        }
    }
}

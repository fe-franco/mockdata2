use indicatif::{MultiProgress, ProgressBar};
use rand::{self, Rng};
use reqwest::Client;
use std::{env::current_dir, sync::Arc};

pub(crate) fn random_cpf() -> u64 {
    // returns a valid CPF
    let mut rng = rand::thread_rng();
    let cpf: u64 = format!(
        "{:03}{:03}{:03}{:02}",
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..99)
    )
    .parse()
    .unwrap();
    cpf
}

pub(crate) fn random_rg() -> u64 {
    // returns a valid RG
    let mut rng = rand::thread_rng();
    let rg: u64 = format!(
        "{:03}{:03}{:03}{:02}",
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..999),
        rng.gen_range(0..99)
    )
    .parse()
    .unwrap();
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

pub(crate) fn random_br_phone() -> u64 {
    let mut rng = rand::thread_rng();

    // Generate a number between 9000_0000 and 9999_9999
    let phone_number: u64 = rng.gen_range(9_000_0000..=9_999_9999);

    phone_number
}

pub(crate) fn random_cep() -> u64 {
    // returns a valid BR CEP
    let mut rng = rand::thread_rng();
    let cep: u64 = format!("{:05}{:03}", rng.gen_range(0..99999), rng.gen_range(0..999))
        .parse()
        .unwrap();
    cep
}

pub(crate) async fn fetch_data<T: for<'a> serde::de::Deserialize<'a>>(
    client: &Client,
    url: &str,
) -> Result<T, anyhow::Error> {
    let body = client.get(url).send().await?.text().await?;
    Ok(serde_json::from_str(&body)?)
}

pub(crate) async fn _fetch_with_exponential_backoff<T: for<'a> serde::de::Deserialize<'a>>(
    client: &Client,
    url: &str,
) -> Result<T, anyhow::Error> {
    let mut delay = 5; // Start with a 5-second delay

    loop {
        match fetch_data::<T>(&client, &url).await {
            Ok(result) => return Ok(result),
            Err(error) => {
                // println!("Error: {}", url);
                if delay > 5 {
                    // println!("This is taking too long, skipping...");
                    // println!(
                    //     "last returned body: {}",
                    //     client.get(url).send().await?.text().await?
                    // );
                    // println!("last error: {}", error);
                    // skip this url
                    return Err(anyhow::Error::msg(error));
                }

                // print!("\r"); // Clear the line after the countdown

                // Double the delay for the next iteration
                delay *= 2;
            }
        }
    }
}

pub(crate) fn format_number(mut number: i64) -> String {
    let mut suffix = "";
    if number < 0 {
        number = number.abs();
        suffix = "-";
    }
    if number >= 1_000_000_000_000_000 {
        number /= 1_000_000_000_000_000;
        suffix = "Q";
    } else if number >= 1_000_000_000_000 {
        number /= 1_000_000_000_000;
        suffix = "T";
    } else if number >= 1_000_000_000 {
        number /= 1_000_000_000;
        suffix = "B";
    } else if number >= 1_000_000 {
        number /= 1_000_000;
        suffix = "M";
    } else if number >= 1_000 {
        number /= 1_000;
        suffix = "K";
    }

    format!("{:.2}{}", number as f64, suffix)
}

pub(crate) struct ProgressBarHelper {
    pub(crate) pb: ProgressBar,
    total: usize,
}

impl ProgressBarHelper {
    pub(crate) fn new(m: Arc<MultiProgress>, total: usize, prefix: String) -> Self {
        let pb = m.add(ProgressBar::new(total as u64));
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template(
                    "{prefix} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        pb.set_prefix(prefix);

        Self { pb, total }
    }

    pub(crate) fn finish(self) {
        self.pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{prefix} {msg:.green} {elapsed_precise:.green}")
                .unwrap()
                .progress_chars("#>-"),
        );

        self.pb.finish_with_message(format!("{} entries in", {
            format_number(self.total.try_into().unwrap())
        }));
    }
}

pub(crate) fn current_timestamp() -> String {
    let mut curent_date = chrono::Local::now().to_string(); // 2023-11-02 11:27:08.122216600 -03:00
    curent_date = curent_date[..19].to_string();

    // TO_DATE('2023-11-02 11:27:08', 'YYYY-MM-DD HH24:MI:SS')
    format!("TO_DATE('{}', 'YYYY-MM-DD HH24:MI:SS')", curent_date)
}

pub(crate) fn create_data_dir() {
    // check if data dir exists in current dir
    // if not, create it
    let mut data_dir = current_dir().unwrap();
    data_dir.push("data");
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).unwrap();
    }
}

pub(crate) fn format_time(time: u64) -> String {
    let mut time = time;
    let mut suffix = "";
    if time <= 60 {
        suffix = "s";
    }
    if time >= 60 {
        time /= 60;
        suffix = "m";
    }
    if time >= 60 {
        time /= 60;
        suffix = "h";
    }
    if time >= 24 {
        time /= 24;
        suffix = "d";
    }
    if time >= 7 {
        time /= 7;
        suffix = "w";
    }

    format!("{}{}", time, suffix)
}

use std::ops::{Bound, RangeBounds};

pub(crate) trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

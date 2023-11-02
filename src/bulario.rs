use rand::seq::SliceRandom;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://consultas.anvisa.gov.br";

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Category {
    pub(crate) id: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[allow(non_snake_case)]
pub(crate) struct Medication {
    pub(crate) idProduto: u64,
    pub(crate) numeroRegistro: String,
    pub(crate) nomeProduto: String,
    pub(crate) expediente: String,
    pub(crate) razaoSocial: String,
    pub(crate) cnpj: String,
    pub(crate) numeroTransacao: String,
    pub(crate) data: String,
    pub(crate) numProcesso: String,
    pub(crate) idBulaPacienteProtegido: String,
    pub(crate) idBulaProfissionalProtegido: String,
    pub(crate) dataAtualizacao: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub(crate) struct MedicineApi {
    pub(crate) content: Vec<Medication>,
    pub(crate) totalPages: usize,
    pub(crate) totalElements: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct BularioClient {
    client: Client,
}

impl BularioClient {
    pub fn new() -> Self {
        let client = Client::new();
        BularioClient { client }
    }
    // "accept": "application/json, text/plain, */*",
    // "accept-language": "pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7",
    // "authorization": "Guest",
    // "cache-control": "no-cache",
    // "if-modified-since": "Mon, 26 Jul 1997 05:00:00 GMT",
    // "pragma": "no-cache",
    // "sec-ch-ua-mobile": "?0",
    // "sec-ch-ua-platform": "\"Windows\"",
    // "sec-fetch-dest": "empty",
    // "sec-fetch-mode": "cors",
    // "sec-fetch-site": "same-origin",
    // "cookie": "FGTServer=77E1DC77AE2F953D7ED796A08A630A01A53CF6FE5FD0E106412591871F9A9BBCFBDEA0AD564FD89D3BDE8278200B; FGTServer=77E1DC77AE2F953D7ED796A08A630A01A53CF6FE5FD0E106412591871F9A9BBCFBDEA0AD564FD89D3BDE8278200B; FGTServer=77E1DC77AE2F953D7ED796A08A630A01A53CF6FE5FD0E106412591871F9A9BBCFBDEA0AD564FD89D3BDE8278200B; _pk_id.42.210e=8eca716434ce3237.1690380888.; FGTServer=77E1DC77AE2F953D7ED796A08A630A01A53CF6FE5FD0E106412591871F9A9BBCFBDEA0AD564FD89D3BDE8278200B; _cfuvid=L.SzxLLxZoWYrYqhaiRgS5MTkV77mwE5uIyLNWvyufk-1690462598410-0-604800000; _pk_ref.42.210e=%5B%22%22%2C%22%22%2C1690462669%2C%22https%3A%2F%2Fwww.google.com%2F%22%5D; _pk_ses.42.210e=1; cf_clearance=tk5QcLSYPlUQfr8s2bTGXyvC2KZdHcEIYU8r6HCgNvQ-1690462689-0-160.0.0",
    // "Referer": "https://consultas.anvisa.gov.br/",
    // "UserAgent": utils.randomUseragent(),
    // "Referrer-Policy": "no-referrer-when-downgrade"

    fn headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "accept",
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        headers.insert(
            "accept-language",
            HeaderValue::from_static("pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7"),
        );
        headers.insert("authorization", HeaderValue::from_static("Guest"));
        headers.insert("cache-control", HeaderValue::from_static("no-cache"));
        headers.insert(
            "if-modified-since",
            HeaderValue::from_static("Mon, 26 Jul 1997 05:00:00 GMT"),
        );
        headers.insert("pragma", HeaderValue::from_static("no-cache"));
        headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
        headers.insert(
            "sec-ch-ua-platform",
            HeaderValue::from_static("\"Windows\""),
        );
        headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
        headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
        headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
        headers.insert("cookie",HeaderValue::from_static( "FGTServer=77E1DC77AE2F953D7ED796A08A630A01A53CF6FE5FD0E106412591871F9A9BBCFBDEA0AD564FD89D3BDE8278200B; FGTServer=77E1DC77AE2F953D7ED796A08A630A01A53CF6FE5FD0E106412591871F9A9BBCFBDEA0AD564FD89D3BDE8278200B; FGTServer=77E1DC77AE2F953D7ED796A08A630A01A53CF6FE5FD0E106412591871F9A9BBCFBDEA0AD564FD89D3BDE8278200B; _pk_id.42.210e=8eca716434ce3237.1690380888.; FGTServer=77E1DC77AE2F953D7ED796A08A630A01A53CF6FE5FD0E106412591871F9A9BBCFBDEA0AD564FD89D3BDE8278200B; _cfuvid=L.SzxLLxZoWYrYqhaiRgS5MTkV77mwE5uIyLNWvyufk-1690462598410-0-604800000; _pk_ref.42.210e=%5B%22%22%2C%22%22%2C1690462669%2C%22https%3A%2F%2Fwww.google.com%2F%22%5D; _pk_ses.42.210e=1; cf_clearance=tk5QcLSYPlUQfr8s2bTGXyvC2KZdHcEIYU8r6HCgNvQ-1690462689-0-160.0.0"));
        headers.insert(
            "Referer",
            HeaderValue::from_static("https://consultas.anvisa.gov.br/"),
        );
        headers.insert("UserAgent", HeaderValue::from_static(random_useragent()));
        headers.insert(
            "Referrer-Policy",
            HeaderValue::from_static("no-referrer-when-downgrade"),
        );

        headers
    }

    pub async fn fetch_categories(&self) -> Result<Vec<Category>, anyhow::Error> {
        let url = format!("{}/api/tipoCategoriaRegulatoria", BASE_URL);
        let response = self
            .client
            .get(&url)
            .headers(BularioClient::headers())
            .send()
            .await?
            .text()
            .await?;
        // print!("Response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn fetch_medicines_by_category(
        &self,
        id_categoria: usize,
        pagina: u64,
    ) -> Result<MedicineApi, anyhow::Error> {
        let url = format!(
            "{}/api/consulta/bulario?count=100&filter%5BcategoriasRegulatorias%5D={}&page={}",
            BASE_URL, id_categoria, pagina
        );
        let response = self
            .client
            .get(&url)
            .headers(BularioClient::headers())
            .send()
            .await?
            .text()
            .await?;
        // print!("Response: {}", response);
        Ok(serde_json::from_str(&response)?)
    }
}

fn random_useragent() -> &'static str {
    let agents = [
        "Mozilla/5.0 (Windows NT 6.2; rv:20.0) Gecko/20121202 Firefox/20.0",
        "Mozilla/5.0 (X11; Fedora; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.116 Safari/537.36",
        "Mozilla/5.0 (X11; Linux i686; rv:16.0) Gecko/20100101 Firefox/16.0",
        "Opera/9.80 (X11; Linux i686) Presto/2.12.388 Version/12.16",
        "Mozilla/5.0 (iPad; U; CPU OS 3_2 like Mac OS X; en-us) AppleWebKit/531.21.10 (KHTML, like Gecko) Version/4.0.4 Mobile/7B334b Safari/531.21.10",
        "Mozilla/5.0 (iPad; U; CPU OS 4_2_1 like Mac OS X; ja-jp) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8C148 Safari/6533.18.5",
        "Mozilla/5.0 (Linux; Android 4.4.2; LG-V410 Build/KOT49I.V41010d) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/30.0.1599.103 Safari/537.36",
        "Mozilla/5.0 (Linux; Android 7.0; Moto G (5) Plus Build/NPNS25.137-35-5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.107 Mobile Safari/537.36",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/535.7 (KHTML, like Gecko) Chrome/16.0.912.36 Safari/535.7",
        "Mozilla/5.0 (Windows; U; Windows NT 5.1; en-US) AppleWebKit/531.21.8 (KHTML, like Gecko) Version/4.0.4 Safari/531.21.10"
    ];
    agents.choose(&mut rand::thread_rng()).unwrap_or(&agents[0])
}

use csv::Writer;
use fake::{
    faker::address::en::{StreetName, ZipCode},
    Fake,
};
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

use crate::common::fetch_data;

const STATES_URL: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/estados";
const CITIES_URL: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/municipios";
const NEIGHBORHOODS_URL: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/distritos";
const CREATED_BY: &str = "1";

fn current_timestamp() -> String {
    chrono::Local::now().to_string()
}

fn create_csv_writer(path: &str) -> Result<Writer<File>, csv::Error> {
    csv::Writer::from_path(path)
}

pub(crate) async fn generate_states(client: Client) -> Result<usize, anyhow::Error> {
    println!("Generating states...");
    let json: Vec<UF> = fetch_data(&client, STATES_URL).await?;
    let mut writer = create_csv_writer("data/states.csv")?;
    let mut len: usize = 0;
    for state in json.iter() {
        let state_data = State {
            ID_ESTADO: state.id,
            SG_ESTADO: state.sigla.clone(),
            NM_ESTADO: state.nome.clone(),
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: CREATED_BY.to_string(),
        };
        writer.serialize(&state_data)?;
        len += 1
    }
    Ok(len)
}

pub(crate) async fn generate_cities(client: Client) -> Result<Vec<City>, anyhow::Error> {
    println!("Generating cities...");
    let created_at = chrono::Local::now().to_string();
    let created_by = "1".to_string();
    let mut writer = csv::Writer::from_path("data/cities.csv")?;

    let ibge_code_to_ddd = get_ibge_code_to_ddd()?;
    let json: Vec<Municipio> = fetch_data(&client, CITIES_URL).await?;

    let mut cities = Vec::new();

    for municipio in json.iter() {
        let city_data = City {
            ID_CIDADE: municipio.id,
            ID_ESTADO: municipio.microrregiao.mesorregiao.UF.id,
            NM_CIDADE: municipio.nome.clone(),
            CD_IBGE: municipio.id.to_string(),
            NR_DDD: ibge_code_to_ddd
                .get(&municipio.id.to_string())
                .expect("ddd not found")
                .clone(),
            DT_CADASTRO: created_at.clone(),
            NM_USUARIO: created_by.clone(),
        };

        writer.serialize(&city_data)?;

        cities.push(city_data);
    }
    Ok(cities)
}

fn get_ibge_code_to_ddd() -> Result<HashMap<String, String>, anyhow::Error> {
    let mut ibge_code_to_ddd = HashMap::new();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("Codigos_Nacionais.csv")?;
    for result in reader.deserialize() {
        let record: HashMap<String, String> = result?;
        let ibge_code = record.get("CO_MUNICIPIO").unwrap();
        let ddd = record.get("CN").unwrap();
        ibge_code_to_ddd.insert(ibge_code.to_string(), ddd.to_string());
    }
    Ok(ibge_code_to_ddd)
}

pub(crate) fn get_ddds() -> Result<Vec<usize>, anyhow::Error> {
    let mut ddds = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_path("Codigos_Nacionais.csv")?;
    for result in reader.deserialize() {
        let record: HashMap<String, String> = result?;
        let ddd = record.get("CN").unwrap();
        ddds.push(ddd.parse::<usize>()?);
    }
    Ok(ddds)
}

pub(crate) async fn generate_neighborhoods(
    client: Client,
) -> Result<Vec<Neighborhood>, anyhow::Error> {
    println!("Generating neighborhoods...");
    let created_at = chrono::Local::now().to_string();
    let created_by = "1".to_string();
    let mut writer = csv::Writer::from_path("data/neighborhoods.csv")?;

    let json: Vec<Distrito> = fetch_data(&client, NEIGHBORHOODS_URL).await?;

    let mut neighborhoods: Vec<Neighborhood> = Vec::new();
    for neighborhood in json.iter() {
        let neighborhood_data: Neighborhood = Neighborhood {
            ID_BAIRRO: neighborhood.id,
            ID_CIDADE: neighborhood.municipio.id,
            NM_BAIRRO: neighborhood.nome.clone(),
            NM_ZONA_BAIRRO: [
                "CENTRO",
                "ZONA LESTE",
                "ZONA NORTE",
                "ZONA OESTE",
                "ZONA SUL",
            ]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string(),

            DT_CADASTRO: created_at.clone(),
            NM_USUARIO: created_by.clone(),
        };

        writer.serialize(&neighborhood_data)?;

        neighborhoods.push(neighborhood_data);
    }

    Ok(neighborhoods)
}

pub(crate) fn generate_address(
    neighborhood: &Vec<Neighborhood>,
    total: usize,
) -> Result<Vec<Street>, csv::Error> {
    println!("Generating addresses...");
    let mut writer = csv::Writer::from_path("data/address.csv").unwrap();

    let mut addresses: Vec<Street> = Vec::new();

    for i in 0..total {
        let street_name: String = StreetName().fake();
        let zip_code: String = ZipCode().fake();
        let neighborhood_id: usize = neighborhood
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
            .ID_BAIRRO as usize;

        let address_data: Street = Street {
            ID_LOGRADOURO: i.try_into().expect("cant fit into u32"),
            ID_BAIRRO: neighborhood_id.try_into().expect("cant fit into usize"),
            NM_LOGRADOURO: street_name,
            NR_CEP: zip_code,
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: CREATED_BY.to_string(),
        };

        writer.serialize(&address_data).unwrap();

        addresses.push(address_data);
    }

    Ok(addresses)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Regiao {
    id: u32,
    sigla: String,
    nome: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Microrregiao {
    id: u32,
    nome: String,
    mesorregiao: Mesorregiao,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct Mesorregiao {
    id: u32,
    nome: String,
    UF: UF,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UF {
    id: u32,
    sigla: String,
    nome: String,
    regiao: Regiao,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Municipio {
    id: u32,
    nome: String,
    microrregiao: Microrregiao,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Distrito {
    id: u32,
    nome: String,
    municipio: Municipio,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
struct State {
    ID_ESTADO: u32,
    SG_ESTADO: String,
    NM_ESTADO: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(crate) struct City {
    pub(crate) ID_CIDADE: u32,
    pub(crate) ID_ESTADO: u32,
    NM_CIDADE: String,
    CD_IBGE: String,
    pub(crate) NR_DDD: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(crate) struct Neighborhood {
    pub(crate) ID_BAIRRO: u32,
    pub(crate) ID_CIDADE: u32,
    NM_BAIRRO: String,
    NM_ZONA_BAIRRO: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(crate) struct Street {
    pub(crate) ID_LOGRADOURO: u32,
    ID_BAIRRO: u32,
    NM_LOGRADOURO: String,
    NR_CEP: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

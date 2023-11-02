use fake::{
    faker::address::en::{StreetName, ZipCode},
    Fake,
};
use indicatif::{MultiProgress, ProgressBar};
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use crate::{
    common::{current_timestamp, fetch_data, ProgressBarHelper},
    define_and_impl_sql_insertable,
    sql_generator::SqlGenerator,
};

define_and_impl_sql_insertable!(
    T_RHSTU_ESTADO {
        pub ID_ESTADO: u32,
        SG_ESTADO: String,
        NM_ESTADO: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_CIDADE {
        pub ID_CIDADE: u32,
        pub ID_ESTADO: u32,
        NM_CIDADE: String,
        CD_IBGE: u32,
        pub NR_DDD: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_BAIRRO {
        pub ID_BAIRRO: u32,
        pub ID_CIDADE: u32,
        NM_BAIRRO: String,
        NM_ZONA_BAIRRO: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_LOGRADOURO {
        pub ID_LOGRADOURO: u32,
        pub ID_BAIRRO: u32,
        NM_LOGRADOURO: String,
        NR_CEP: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    }
);

const STATES_URL: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/estados";
const CITIES_URL: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/municipios";
const NEIGHBORHOODS_URL: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/distritos";
const CREATED_BY: &str = "1";

pub(crate) async fn generate_states(
    client: Client,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Result<usize, anyhow::Error> {
    let json: Vec<UF> = fetch_data(&client, STATES_URL).await?;

    let mut len: usize = 0;
    let mut states: Vec<T_RHSTU_ESTADO> = Vec::with_capacity(json.len());

    let pb_helper = ProgressBarHelper::new(m, json.len(), "States:".to_string());
    let pb = &pb_helper.pb;

    for state in json.iter() {
        let state_data = T_RHSTU_ESTADO {
            ID_ESTADO: state.id,
            SG_ESTADO: state.sigla.clone(),
            NM_ESTADO: state.nome.clone(),
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: CREATED_BY.to_string(),
        };
        states.push(state_data);
        len += 1;
        pb.inc(1);
        main_pb.inc(1);
    }

    // Use SqlGenerator to generate SQL and write to a file
    let generator = SqlGenerator::new(states);
    generator.write_to_file()?;

    pb_helper.finish();

    Ok(len)
}

pub(crate) async fn generate_cities(
    client: Client,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Result<Vec<T_RHSTU_CIDADE>, anyhow::Error> {
    let created_at = current_timestamp();
    let created_by = "1".to_string();

    let ibge_code_to_ddd = get_ibge_code_to_ddd()?;
    let json: Vec<Municipio> = fetch_data(&client, CITIES_URL).await?;

    let mut cities = Vec::new();

    let pb_helper = ProgressBarHelper::new(m, json.len(), "Cities:".to_string());
    let pb = &pb_helper.pb;

    for municipio in json.iter() {
        let city_data = T_RHSTU_CIDADE {
            ID_CIDADE: municipio.id,
            ID_ESTADO: municipio.microrregiao.mesorregiao.UF.id,
            NM_CIDADE: municipio.nome.clone(),
            CD_IBGE: municipio.id,
            NR_DDD: ibge_code_to_ddd
                .get(&municipio.id.to_string())
                .expect("ddd not found")
                .clone(),
            DT_CADASTRO: created_at.clone(),
            NM_USUARIO: created_by.clone(),
        };

        cities.push(city_data);
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(cities.clone());
    generator.write_to_file()?;

    pb_helper.finish();

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
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Result<Vec<T_RHSTU_BAIRRO>, anyhow::Error> {
    // println!("Generating neighborhoods...");
    let created_at = current_timestamp();
    let created_by = "1".to_string();

    let json: Vec<Distrito> = fetch_data(&client, NEIGHBORHOODS_URL).await?;

    let mut neighborhoods: Vec<T_RHSTU_BAIRRO> = Vec::new();

    let pb_helper = ProgressBarHelper::new(m, json.len(), "Neighborhoods:".to_string());
    let pb = &pb_helper.pb;

    for neighborhood in json.iter() {
        let neighborhood_data: T_RHSTU_BAIRRO = T_RHSTU_BAIRRO {
            ID_BAIRRO: pb.position().try_into().expect("cant fit into u32"),
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

        neighborhoods.push(neighborhood_data);
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(neighborhoods.clone());
    generator.write_to_file()?;

    pb_helper.finish();

    Ok(neighborhoods)
}

pub(crate) fn generate_address(
    neighborhood: &Vec<T_RHSTU_BAIRRO>,
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Result<Vec<T_RHSTU_LOGRADOURO>, csv::Error> {
    // println!("Generating addresses...");

    let mut addresses: Vec<T_RHSTU_LOGRADOURO> = Vec::new();

    let pb_helper = ProgressBarHelper::new(m, total, "Addresses:".to_string());
    let pb = &pb_helper.pb;

    for i in 0..total {
        let street_name: String = StreetName().fake();
        let zip_code: String = ZipCode().fake();
        let neighborhood_id: usize = neighborhood
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
            .ID_BAIRRO as usize;

        let address_data: T_RHSTU_LOGRADOURO = T_RHSTU_LOGRADOURO {
            ID_LOGRADOURO: i.try_into().expect("cant fit into u32"),
            ID_BAIRRO: neighborhood_id.try_into().expect("cant fit into usize"),
            NM_LOGRADOURO: street_name,
            NR_CEP: zip_code,
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: CREATED_BY.to_string(),
        };

        addresses.push(address_data);
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(addresses.clone());
    generator.write_to_file()?;

    pb_helper.finish();

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

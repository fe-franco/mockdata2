// - Code Take-Home Test: Hospital Data Simulation
// Objective:
// Your task is to simulate a hospital database by generating a dataset with fictional but realistic data for various tables related to a hospital system. The test aims to evaluate your ability to model relational data, generate realistic test data, and manage dependencies between tables.

// Task:
// 1. Data Tables:
// - You will be required to generate data for the following tables:
// - T_RHSTU_ESTADO - "ID_ESTADO","SG_ESTADO","NM_ESTADO","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_CIDADE - "ID_CIDADE","ID_ESTADO","NM_CIDADE","CD_IBGE","NR_DDD","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_BAIRRO - "ID_BAIRRO","ID_CIDADE","NM_BAIRRO","NM_ZONA_BAIRRO","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_LOGRADOURO - "ID_LOGRADOURO","ID_BAIRRO","NM_LOGRADOURO","NR_CEP","DT_CADASTRO","NM_USUARIO"

// - T_RHSTU_UNID_HOSPITALAR - "ID_UNID_HOSPITAL","NM_UNID_HOSPITALAR","NM_RAZAO_SOCIAL_UNID_HOSP","DT_FUNDACAO","NR_LOGRADOURO","DS_COMPLEMENTO_NUMERO","DS_PONTO_REFERENCIA","DT_INICIO","DT_TERMINO","DT_CADASTRO","NM_USUARIO"

// - T_RHSTU_FUNCIONARIO - "ID_FUNC","ID_SUPERIOR","NM_FUNC","DS_CARGO","DT_NASCIMENTO","VL_SALARIO","NR_RG","NR_CPF","ST_FUNC","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_MEDICO - "ID_FUNC","NR_CRM","DS_ESPECIALIDADE","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_MOTORISTA - "ID_FUNC","NR_CNH","NM_CATEGORIA_CNH","DT_VALIDADE_CNH","DT_CADASTRO","NM_USUARIO"

// - T_RHSTU_PACIENTE - "ID_PACIENTE","NM_PACIENTE","NR_CPF","NM_RG","DT_NASCIMENTO","FL_SEXO_BIOLOGICO","DS_ESCOLARIDADE","DS_ESTADO_CIVIL","NM_GRUPO_SANGUINEO","NR_ALTURA","NR_PESO","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_TIPO_CONTATO - "ID_TIPO_CONTATO","NM_TIPO_CONTATO","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_CONTATO_PACIENTE - "ID_PACIENTE","ID_CONTATO","ID_TIPO_CONTATO","NM_CONTATO","NR_DDI","NR_DDD","NR_TELEFONE","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_EMAIL_PACIENTE - "ID_EMAIL","ID_PACIENTE","DS_EMAIL","TP_EMAIL","ST_EMAIL","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_TELEFONE_PACIENTE - "ID_PACIENTE","ID_TELEFONE","NR_DDI","NR_DDD","NR_TELEFONE","TP_TELEFONE","ST_TELEFONE","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_ENDERECO_PACIENTE - "ID_ENDERECO","ID_PACIENTE","ID_LOGRADOURO","NR_LOGRADOURO","DS_COMPLEMENTO_NUMERO","DS_PONTO_REFERENCIA","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"

// - T_RHSTU_PLANO_SAUDE - "ID_PLANO_SAUDE","DS_RAZAO_SOCIAL","NM_FANTASIA_PLANO_SAUDE","DS_PLANO_SAUDE","NR_CNPJ","NM_CONTATO","DS_TELEFONE","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_PACIENTE_PLANO_SAUDE - "ID_PACIENTE_PS","ID_PACIENTE","ID_PLANO_SAUDE","NR_CARTEIRA_PS","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"

// - T_RHSTU_CONSULTA - "ID_UNID_HOSPITAL","ID_CONSULTA","ID_PACIENTE","ID_FUNC","DT_HR_CONSULTA","NR_CONSULTORIO","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_MEDICAMENTO - "ID_MEDICAMENTO","NM_MEDICAMENTO","DS_DETALHADA_MEDICAMENTO","NR_CODIGO_BARRAS","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_FORMA_PAGAMENTO - "ID_FORMA_PAGTO","NM_FORMA_PAGTO","DS_FORMA_PAGTO","ST_FORMA_PAGTO","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_CONSULTA_FORMA_PAGTO - "ID_CONSULTA_FORMA_PAGTO","ID_UNID_HOSPITAL","ID_CONSULTA","ID_PACIENTE_PS","ID_FORMA_PAGTO","DT_PAGTO_CONSULTA","ST_PAGTO_CONSULTA","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_PRESCRICAO_MEDICA - "ID_PRESCRICAO_MEDICA","ID_UNID_HOSPITAL","ID_CONSULTA","ID_MEDICAMENTO","DS_POSOLOGIA","DS_VIA","DS_OBSERVACAO_USO","QT_MEDICAMENTO","NM_USUARIO","DT_CADASTRO"

// - T_RHSTU_ENDERECO_UNIDHOSP - "ID_END_UNIDHOSP","ID_UNID_HOSPITAL","ID_LOGRADOURO","NR_LOGRADOURO","DS_COMPLEMENTO_NUMERO","DS_PONTO_REFERENCIA","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"

// 2. Requirements:
// Create a standalone program or script that, when run, generates .csv files for each table.

// Ensure that data dependencies are maintained. For instance, every city (from T_RHSTU_CIDADE) should belong to a valid state (from T_RHSTU_ESTADO).

// Make use of libraries or tools you're familiar with, such as Faker for data generation.

// It's not about the quantity, but the quality of data and the relationships between them. A smaller, well-structured dataset is preferred over a large, unstructured one.
// That being said the goal is to generate at least 10 million rows of data in total (across all tables).

mod common;
mod consultation;
mod geography;
mod health_plan;
mod hospital;
mod medicine;
mod patient;

use crate::geography::{
    generate_address, generate_cities, generate_neighborhoods, generate_states,
};
use crate::medicine::get_medicines;
use crate::patient::generate_patients;
use reqwest::Client;

const TOTAL_ENTRIES: usize = 10000000;
use anyhow::Result; // Use anyhow for better error handling

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = Client::new();

    // // Spawn each function as a separate task
    // let states_task = tokio::spawn(generate_states(client.clone()));
    // let cities_task = tokio::spawn(generate_cities(client.clone()));
    // let neighborhoods_task = tokio::spawn(generate_neighborhoods(client.clone()));

    // // Await all tasks concurrently
    // let (states, cities, neighborhoods) =
    //     tokio::try_join!(states_task, cities_task, neighborhoods_task)?;

    // // Unwrap the Result values
    // let states = states?;
    // let cities = cities?;
    // let neighborhoods = neighborhoods?;
    // let address = generate_address(&neighborhoods, states + cities + neighborhoods.len())?;

    let mut entries = 9967466;
    // TOTAL_ENTRIES - ((states + cities + neighborhoods.len() + address.len()) as usize);

    // let mut patients = generate_patients((entries as f32 * 0.4) as usize);
    let medicine_len = get_medicines().await;

    // entries -= patients.len();

    println!(
        "entries left: {} | entries generated: {} | percentage: {}%",
        entries,
        TOTAL_ENTRIES - entries,
        (TOTAL_ENTRIES - entries) as f32 / TOTAL_ENTRIES as f32 * 100.0
    );

    Ok(())
}

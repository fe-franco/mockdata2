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

mod bulario;
mod common;
mod constants;
mod sql_generator;
mod tables;

use crate::tables::geography::{
    generate_address, generate_cities, generate_neighborhoods, generate_states,
};
use crate::tables::medicine::{generate_medical_prescription, get_medicines};
use crate::tables::patient::generate_patients;
use anyhow::Result;
use common::{create_data_dir, format_number, format_time};
use constants::{
    T_RHSTU_CONSULTA_FORMA_PAGTO_ROWS, T_RHSTU_CONSULTA_ROWS, T_RHSTU_CONTATO_PACIENTE_ROWS,
    T_RHSTU_EMAIL_PACIENTE_ROWS, T_RHSTU_ENDERECO_PACIENTE_ROWS, T_RHSTU_ENDERECO_UNIDHOSP_ROWS,
    T_RHSTU_FUNCIONARIO_ROWS, T_RHSTU_LOGRADOURO_ROWS, T_RHSTU_MEDICO_ROWS, T_RHSTU_MOTORISTA_ROWS,
    T_RHSTU_PACIENTE_PLANO_SAUDE_ROWS, T_RHSTU_PACIENTE_ROWS, T_RHSTU_PLANO_SAUDE_ROWS,
    T_RHSTU_PRESCRICAO_MEDICA_ROWS, T_RHSTU_TELEFONE_PACIENTE_ROWS, T_RHSTU_UNID_HOSPITALAR_ROWS,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::sync::{Arc, Mutex};
use tables::hospital::generate_employee; // Use anyhow for better error handling

const TOTAL_ENTRIES: usize = 10_000_000;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    create_data_dir();
    let m = Arc::new(MultiProgress::new());
    let pb = Arc::new(m.add(ProgressBar::new(TOTAL_ENTRIES as u64)));
    pb.set_prefix("Total:");
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{prefix} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let client = Client::new();

    // Initial tasks
    let states_task = tokio::spawn(generate_states(client.clone(), m.clone(), pb.clone()));
    let cities_task = tokio::spawn(generate_cities(client.clone(), m.clone(), pb.clone()));
    let neighborhoods_task = tokio::spawn(generate_neighborhoods(
        client.clone(),
        m.clone(),
        pb.clone(),
    ));
    let medicines_task = tokio::spawn(get_medicines(m.clone(), pb.clone()));

    let (cities, neighborhoods) = tokio::try_join!(cities_task, neighborhoods_task)?;

    let cities = cities?;
    let neighborhoods = neighborhoods?;

    let address = generate_address(
        &neighborhoods,
        T_RHSTU_LOGRADOURO_ROWS as usize,
        m.clone(),
        pb.clone(),
    )?;

    // Patient-related tasks

    let patients_task = tokio::spawn(generate_patients(
        T_RHSTU_PACIENTE_ROWS as usize,
        m.clone(),
        pb.clone(),
    ));
    let patient_address_task = tokio::spawn(tables::patient::generate_patients_addresses(
        T_RHSTU_ENDERECO_PACIENTE_ROWS as usize,
        address.clone(),
        m.clone(),
        pb.clone(),
    ));
    let contact_types_task = tokio::spawn(tables::patient::generate_contact_types(
        6,
        m.clone(),
        pb.clone(),
    ));
    let contact_types = tokio::try_join!(contact_types_task)?;

    let patient_contact = tokio::spawn(tables::patient::generate_patient_contacts(
        T_RHSTU_CONTATO_PACIENTE_ROWS as usize,
        contact_types.0,
        m.clone(),
        pb.clone(),
    ));
    let patient_email_task = tokio::spawn(tables::patient::generate_emails(
        T_RHSTU_EMAIL_PACIENTE_ROWS as usize,
        m.clone(),
        pb.clone(),
    ));

    let patient_telefone_task = tokio::spawn(tables::patient::generate_telephones(
        T_RHSTU_TELEFONE_PACIENTE_ROWS as usize,
        m.clone(),
        pb.clone(),
    ));

    // Hospital-related tasks
    let hospitals_task = tokio::spawn(tables::hospital::generate_hospital(
        T_RHSTU_UNID_HOSPITALAR_ROWS as usize,
        m.clone(),
        pb.clone(),
    ));

    let hospital_address_taks = tokio::spawn(tables::hospital::generate_hospital_address(
        T_RHSTU_ENDERECO_UNIDHOSP_ROWS as usize,
        neighborhoods,
        cities,
        m.clone(),
        pb.clone(),
    ));

    let employees_task = tokio::spawn(tables::hospital::generate_employee(
        T_RHSTU_FUNCIONARIO_ROWS as usize,
        m.clone(),
        pb.clone(),
    ));

    let employee_ids: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(employees_task.await?));

    let doctors_task = tokio::spawn(tables::hospital::generate_doctor(
        employee_ids.clone(),
        T_RHSTU_MEDICO_ROWS as usize,
        m.clone(),
        pb.clone(),
    ));

    let drivers_task = tokio::spawn(tables::hospital::generate_driver(
        employee_ids,
        T_RHSTU_MOTORISTA_ROWS as usize,
        m.clone(),
        pb.clone(),
    ));

    // Consultation-related tasks
    let consultations_task = tokio::spawn(tables::consultation::generate_consultations(
        T_RHSTU_CONSULTA_ROWS as usize,
        T_RHSTU_UNID_HOSPITALAR_ROWS as usize,
        T_RHSTU_PACIENTE_ROWS as usize,
        m.clone(),
        pb.clone(),
    ));
    let payment_methods_task = tokio::spawn(tables::consultation::generate_payment_methods(
        6,
        m.clone(),
        pb.clone(),
    ));
    let (consultations, payment_methods) =
        tokio::try_join!(consultations_task, payment_methods_task)?;

    let consultation_payment_methods_task =
        tokio::spawn(tables::consultation::generate_consultation_payment_methods(
            T_RHSTU_CONSULTA_FORMA_PAGTO_ROWS as usize,
            payment_methods,
            consultations,
            m.clone(),
            pb.clone(),
        ));

    // Health plan-related tasks
    let health_plans = tables::health_plan::generate_health_plans(
        T_RHSTU_PLANO_SAUDE_ROWS as usize,
        m.clone(),
        pb.clone(),
    );

    let health_plan_patient_task =
        tokio::spawn(tables::health_plan::generate_patient_health_plans(
            T_RHSTU_PACIENTE_PLANO_SAUDE_ROWS as usize,
            health_plans,
            T_RHSTU_PACIENTE_ROWS as usize,
            m.clone(),
            pb.clone(),
        ));

    // Medicine-related tasks
    let medicines = medicines_task.await?;

    let generate_medical_prescription_task = tokio::spawn(generate_medical_prescription(
        T_RHSTU_PRESCRICAO_MEDICA_ROWS as usize,
        medicines,
        m.clone(),
        pb.clone(),
    ));

    // await all tasks
    tokio::try_join!(
        states_task,
        patients_task,
        patient_address_task,
        patient_contact,
        patient_email_task,
        patient_telefone_task,
        hospitals_task,
        hospital_address_taks,
        doctors_task,
        drivers_task,
        consultation_payment_methods_task,
        health_plan_patient_task,
        generate_medical_prescription_task
    )?;

    let mut generated_entries = pb.position() as i32;
    let mut discrepancy = TOTAL_ENTRIES as i32 - generated_entries;

    // Here, you can adjust one of the categories to make up for the discrepancy. For simplicity, let's adjust the number of patients:
    if discrepancy > 0 {
        generate_employee(discrepancy as usize, m.clone(), pb.clone()).await;
    }

    generated_entries = pb.position() as i32;
    discrepancy = TOTAL_ENTRIES as i32 - generated_entries;
    let pb_final_time = pb.elapsed();

    // Final progress bar
    let r#final = format!(
        "entries left: {} | entries generated: {} | percentage: {}% | time elapsed: {}",
        format!("{}", format_number(discrepancy.into())),
        format_number(generated_entries.into()),
        format_number(
            generated_entries
                .checked_div(TOTAL_ENTRIES.try_into().unwrap())
                .unwrap_or(0)
                .checked_mul(100)
                .unwrap_or(0)
                .into()
        ),
        format_time(pb_final_time.as_secs())
    );

    let final_pb = m.add(ProgressBar::new(1));
    final_pb.set_style(
        ProgressStyle::default_bar()
            .template("{prefix} {msg:.green}")
            .unwrap()
            .progress_chars("#>-"),
    );
    final_pb.set_prefix("Final:");
    final_pb.finish_with_message(r#final);

    Ok(())
}

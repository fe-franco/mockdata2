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
mod consultation;
mod geography;
mod health_plan;
mod hospital;
mod medicine;
mod patient;

use crate::geography::{
    generate_address, generate_cities, generate_neighborhoods, generate_states,
};
use crate::medicine::{generate_medical_prescription, get_medicines};
use crate::patient::generate_patients;
use anyhow::Result;
use common::{create_data_dir, format_number, format_time};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::sync::{Arc, Mutex}; // Use anyhow for better error handling

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

    let (states, cities, neighborhoods) =
        tokio::try_join!(states_task, cities_task, neighborhoods_task)?;

    let states = states?;
    let cities = cities?;
    let neighborhoods = neighborhoods?;

    let address = generate_address(
        &neighborhoods,
        states + cities.len() + neighborhoods.len(),
        m.clone(),
        pb.clone(),
    )?;

    let initial_entries = states + cities.len() + neighborhoods.len() + address.len();
    let mut remaining_entries = TOTAL_ENTRIES - initial_entries;

    // Patient-related tasks
    let total_patients_contact_types = 3;
    let total_patients = (((remaining_entries as f32 * 0.30).round() as usize
        - total_patients_contact_types) as f32
        / 3.0)
        .round() as usize;

    let patients_task = tokio::spawn(generate_patients(total_patients, m.clone(), pb.clone()));
    let patient_address_task = tokio::spawn(patient::generate_patients_addresses(
        total_patients,
        address.clone(),
        m.clone(),
        pb.clone(),
    ));
    let contact_types_task = tokio::spawn(patient::generate_contact_types(
        total_patients_contact_types,
        m.clone(),
        pb.clone(),
    ));
    let contact_types = tokio::try_join!(contact_types_task)?;

    let patient_contact = tokio::spawn(patient::generate_patient_contacts(
        total_patients,
        contact_types.0,
        m.clone(),
        pb.clone(),
    ));
    let patient_email_task = tokio::spawn(patient::generate_emails(
        total_patients,
        m.clone(),
        pb.clone(),
    ));

    let patient_telefone_task = tokio::spawn(patient::generate_telephones(
        total_patients,
        m.clone(),
        pb.clone(),
    ));

    // Hospital-related tasks
    let total_hospitals = 5000;
    remaining_entries -= total_hospitals;
    let hospitals_task = tokio::spawn(hospital::generate_hospital(
        total_hospitals,
        m.clone(),
        pb.clone(),
    ));

    let hospital_address_taks = tokio::spawn(hospital::generate_hospital_address(
        total_hospitals,
        neighborhoods,
        cities,
        m.clone(),
        pb.clone(),
    ));

    let total_employees = (total_hospitals * 200) as usize;
    let total_doctors = (total_employees as f32 * 0.8).round() as usize;
    let total_drivers = total_employees - total_doctors; // Adjust to ensure total_employees = total_doctors + total_drivers

    let employees_task = tokio::spawn(hospital::generate_employee(
        total_employees,
        m.clone(),
        pb.clone(),
    ));

    let employee_ids: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(employees_task.await?));

    let doctors_task = tokio::spawn(hospital::generate_doctor(
        employee_ids.clone(),
        total_doctors,
        m.clone(),
        pb.clone(),
    ));

    let drivers_task = tokio::spawn(hospital::generate_driver(
        employee_ids,
        total_drivers,
        m.clone(),
        pb.clone(),
    ));

    // Consultation-related tasks
    let total_consultations = total_patients; // One consultation per patient for simplicity
    let consultations_task = tokio::spawn(consultation::generate_consultations(
        total_consultations,
        total_hospitals,
        total_patients,
        m.clone(),
        pb.clone(),
    ));
    let payment_methods_task = tokio::spawn(consultation::generate_payment_methods(
        5,
        m.clone(),
        pb.clone(),
    ));
    let (consultations, payment_methods) =
        tokio::try_join!(consultations_task, payment_methods_task)?;

    let consultation_payment_methods_task =
        tokio::spawn(consultation::generate_consultation_payment_methods(
            total_patients,
            payment_methods,
            consultations,
            m.clone(),
            pb.clone(),
        ));

    // Health plan-related tasks
    let total_health_plans = (remaining_entries as f32 * 0.01).round() as usize;
    let health_plans =
        health_plan::generate_health_plans(total_health_plans, m.clone(), pb.clone());

    let health_plan_patient_task = tokio::spawn(health_plan::generate_patient_health_plans(
        total_patients,
        health_plans,
        total_patients,
        m.clone(),
        pb.clone(),
    ));

    // Medicine-related tasks
    let medicines = medicines_task.await?;

    let generate_medical_prescription_task = tokio::spawn(generate_medical_prescription(
        total_patients,
        medicines,
        m.clone(),
        pb.clone(),
    ));

    // await all tasks
    tokio::try_join!(
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
        generate_patients(discrepancy as usize, m.clone(), pb.clone()).await;
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

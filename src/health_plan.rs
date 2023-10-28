use fake::{
    faker::{company::en::CompanyName, name::en::Name},
    Fake,
};
use rand::{seq::SliceRandom, Rng};

use crate::{
    common::{random_br_phone, random_cnpj},
    patient::Patient,
};

// - T_RHSTU_PLANO_SAUDE - "ID_PLANO_SAUDE","DS_RAZAO_SOCIAL","NM_FANTASIA_PLANO_SAUDE","DS_PLANO_SAUDE","NR_CNPJ","NM_CONTATO","DS_TELEFONE","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"
pub(crate) struct HealthPlan {
    ID_PLANO_SAUDE: u32,
    DS_RAZAO_SOCIAL: String,
    NM_FANTASIA_PLANO_SAUDE: String,
    DS_PLANO_SAUDE: String,
    NR_CNPJ: String,
    NM_CONTATO: String,
    DS_TELEFONE: String,
    DT_INICIO: String,
    DT_FIM: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}
// - T_RHSTU_PACIENTE_PLANO_SAUDE - "ID_PACIENTE_PS","ID_PACIENTE","ID_PLANO_SAUDE","NR_CARTEIRA_PS","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"

pub(crate) struct PatientHealthPlan {
    ID_PACIENTE_PS: u32,
    ID_PACIENTE: u32,
    ID_PLANO_SAUDE: u32,
    NR_CARTEIRA_PS: String,
    DT_INICIO: String,
    DT_FIM: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

pub(crate) fn generate_health_plans(total: usize) -> Vec<HealthPlan> {
    let mut health_plans: Vec<HealthPlan> = Vec::with_capacity(total);
    let mut rng = rand::thread_rng();

    for _ in 0..total {
        let health_plan_id = rng.gen_range(1..100) as u32;
        let health_plan_name = CompanyName().fake();
        let health_plan_fantasy_name = CompanyName().fake();
        let health_plan_description = CompanyName().fake();
        let health_plan_cnpj = random_cnpj();
        let health_plan_contact = Name().fake();
        let health_plan_phone = random_br_phone();
        let health_plan_start_date = chrono::Local::now().to_string();
        let health_plan_end_date = chrono::Local::now().to_string();
        let health_plan_register_date = chrono::Local::now().to_string();

        health_plans.push(HealthPlan {
            ID_PLANO_SAUDE: health_plan_id,
            DS_RAZAO_SOCIAL: health_plan_name,
            NM_FANTASIA_PLANO_SAUDE: health_plan_fantasy_name,
            DS_PLANO_SAUDE: health_plan_description,
            NR_CNPJ: health_plan_cnpj,
            NM_CONTATO: health_plan_contact,
            DS_TELEFONE: health_plan_phone,
            DT_INICIO: health_plan_start_date,
            DT_FIM: health_plan_end_date,
            DT_CADASTRO: health_plan_register_date,
            NM_USUARIO: "1".to_string(),
        });
    }

    health_plans
}

pub(crate) fn generate_patient_health_plans(
    total: usize,
    health_plans: Vec<HealthPlan>,
    patients: Vec<Patient>,
) -> Vec<PatientHealthPlan> {
    let mut patient_health_plans: Vec<PatientHealthPlan> = Vec::with_capacity(total);
    let mut rng = rand::thread_rng();

    for _ in 0..total {
        let patient_health_plan_id = rng.gen_range(1..100) as u32;
        let patient_health_plan_patient_id = patients
            .choose(&mut rand::thread_rng())
            .unwrap()
            .ID_PACIENTE;
        let patient_health_plan_health_plan_id = health_plans
            .choose(&mut rand::thread_rng())
            .unwrap()
            .ID_PLANO_SAUDE;
        let patient_health_plan_card_number = rand::thread_rng().gen_range(1..100).to_string();
        let patient_health_plan_start_date = chrono::Local::now().to_string();
        let patient_health_plan_end_date = chrono::Local::now().to_string();
        let patient_health_plan_register_date = chrono::Local::now().to_string();
        let patient_health_plan_user_name = "1".to_string();

        patient_health_plans.push(PatientHealthPlan {
            ID_PACIENTE_PS: patient_health_plan_id,
            ID_PACIENTE: patient_health_plan_patient_id,
            ID_PLANO_SAUDE: patient_health_plan_health_plan_id,
            NR_CARTEIRA_PS: patient_health_plan_card_number,
            DT_INICIO: patient_health_plan_start_date,
            DT_FIM: patient_health_plan_end_date,
            DT_CADASTRO: patient_health_plan_register_date,
            NM_USUARIO: patient_health_plan_user_name,
        });
    }

    patient_health_plans
}

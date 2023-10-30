use fake::{
    faker::{company::en::CompanyName, name::en::Name},
    Fake,
};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::common::{random_br_phone, random_cnpj};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
// - T_RHSTU_PLANO_SAUDE - "ID_PLANO_SAUDE","DS_RAZAO_SOCIAL","NM_FANTASIA_PLANO_SAUDE","DS_PLANO_SAUDE","NR_CNPJ","NM_CONTATO","DS_TELEFONE","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"
pub(crate) struct HealthPlan {
    ID_PLANO_SAUDE: u32,
    DS_RAZAO_SOCIAL: String,
    NM_FANTASIA_PLANO_SAUDE: String,
    DS_PLANO_SAUDE: String,
    NR_CNPJ: String,
    NM_CONTATO: String,
    DS_TELEFONE: u32,
    DT_INICIO: String,
    DT_FIM: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}
// - T_RHSTU_PACIENTE_PLANO_SAUDE - "ID_PACIENTE_PS","ID_PACIENTE","ID_PLANO_SAUDE","NR_CARTEIRA_PS","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
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
    let mut writer = csv::Writer::from_path("data/health_plan.csv").unwrap();

    for i in 0..total {
        let health_plan = HealthPlan {
            ID_PLANO_SAUDE: i as u32,
            DS_RAZAO_SOCIAL: CompanyName().fake(),
            NM_FANTASIA_PLANO_SAUDE: CompanyName().fake(),
            DS_PLANO_SAUDE: CompanyName().fake(),
            NR_CNPJ: random_cnpj(),
            NM_CONTATO: Name().fake(),
            DS_TELEFONE: random_br_phone(),
            DT_INICIO: chrono::Local::now().to_string(),
            DT_FIM: chrono::Local::now().to_string(),
            DT_CADASTRO: chrono::Local::now().to_string(),
            NM_USUARIO: "1".to_string(),
        };

        writer.serialize(&health_plan).unwrap();

        health_plans.push(health_plan);
    }

    health_plans
}

pub(crate) async fn generate_patient_health_plans(
    total: usize,
    health_plans: Vec<HealthPlan>,
    total_patients: usize,
) -> Vec<PatientHealthPlan> {
    let mut patient_health_plans: Vec<PatientHealthPlan> = Vec::with_capacity(total);
    let mut rng = rand::thread_rng();

    let mut writer = csv::Writer::from_path("data/patient_health_plan.csv").unwrap();

    for i in 0..total {
        let patient_health_plan = PatientHealthPlan {
            ID_PACIENTE_PS: i as u32,
            ID_PACIENTE: rng.gen_range(1..total_patients) as u32,
            ID_PLANO_SAUDE: health_plans
                .choose(&mut rand::thread_rng())
                .unwrap()
                .ID_PLANO_SAUDE,
            NR_CARTEIRA_PS: rng.gen_range(1..100).to_string(),
            DT_INICIO: chrono::Local::now().to_string(),
            DT_FIM: chrono::Local::now().to_string(),
            DT_CADASTRO: chrono::Local::now().to_string(),
            NM_USUARIO: "1".to_string(),
        };

        writer.serialize(&patient_health_plan).unwrap();

        patient_health_plans.push(patient_health_plan);
    }

    patient_health_plans
}

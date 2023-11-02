use std::sync::Arc;

use fake::{
    faker::{company::en::CompanyName, name::en::Name},
    Fake,
};
use indicatif::{MultiProgress, ProgressBar};
use rand::{seq::SliceRandom, Rng};

use crate::{
    common::{current_timestamp, random_br_phone, random_cnpj, ProgressBarHelper},
    define_and_impl_sql_insertable,
    sql_generator::SqlGenerator,
};

// - T_RHSTU_PLANO_SAUDE - "ID_PLANO_SAUDE","DS_RAZAO_SOCIAL","NM_FANTASIA_PLANO_SAUDE","DS_PLANO_SAUDE","NR_CNPJ","NM_CONTATO","DS_TELEFONE","DT_INICIO","DT_FIM","DT_CADASTRO","NM_USUARIO"
define_and_impl_sql_insertable!(
    T_RHSTU_PLANO_SAUDE {
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
        NM_USUARIO: String
    },
    T_RHSTU_PACIENTE_PLANO_SAUDE {
        ID_PACIENTE_PS: u32,
        ID_PACIENTE: u32,
        ID_PLANO_SAUDE: u32,
        NR_CARTEIRA_PS: String,
        DT_INICIO: String,
        DT_FIM: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    }
);

pub(crate) fn generate_health_plans(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_PLANO_SAUDE> {
    // println!("Generating health plans...");
    let mut health_plans: Vec<T_RHSTU_PLANO_SAUDE> = Vec::with_capacity(total);

    let pb_helper = ProgressBarHelper::new(m, total * 2, "Health Plans:".to_string());
    let pb = &pb_helper.pb;

    for i in 0..total {
        let health_plan = T_RHSTU_PLANO_SAUDE {
            ID_PLANO_SAUDE: i as u32,
            DS_RAZAO_SOCIAL: CompanyName().fake(),
            NM_FANTASIA_PLANO_SAUDE: CompanyName().fake(),
            DS_PLANO_SAUDE: CompanyName().fake(),
            NR_CNPJ: random_cnpj(),
            NM_CONTATO: Name().fake(),
            DS_TELEFONE: random_br_phone(),
            DT_INICIO: current_timestamp(),
            DT_FIM: current_timestamp(),
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: "1".to_string(),
        };

        health_plans.push(health_plan);
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(health_plans.clone());
    let _ = generator.write_to_file(pb);

    pb_helper.finish();

    health_plans
}

pub(crate) async fn generate_patient_health_plans(
    total: usize,
    health_plans: Vec<T_RHSTU_PLANO_SAUDE>,
    total_patients: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_PACIENTE_PLANO_SAUDE> {
    // println!("Generating patient health plans...");
    let mut patient_health_plans: Vec<T_RHSTU_PACIENTE_PLANO_SAUDE> = Vec::with_capacity(total);
    let mut rng = rand::thread_rng();

    let pb_helper = ProgressBarHelper::new(m, total * 2, "Patient Health Plans:".to_string());
    let pb = &pb_helper.pb;

    for i in 0..total {
        let patient_health_plan = T_RHSTU_PACIENTE_PLANO_SAUDE {
            ID_PACIENTE_PS: i as u32,
            ID_PACIENTE: rng.gen_range(1..total_patients) as u32,
            ID_PLANO_SAUDE: health_plans
                .choose(&mut rand::thread_rng())
                .unwrap()
                .ID_PLANO_SAUDE,
            NR_CARTEIRA_PS: rng.gen_range(1..100).to_string(),
            DT_INICIO: current_timestamp(),
            DT_FIM: current_timestamp(),
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: "1".to_string(),
        };

        patient_health_plans.push(patient_health_plan);
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(patient_health_plans.clone());
    let _ = generator.write_to_file(pb);

    pb_helper.finish();

    patient_health_plans
}

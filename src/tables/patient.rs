use crate::common::{random_br_phone, random_cpf, random_rg, ProgressBarHelper};
use crate::define_and_impl_sql_insertable;
use crate::sql_generator::SqlGenerator;
use crate::tables::geography::{get_ddds, TRHSTU_LOGRADOURO};
use fake::faker::internet::en::FreeEmail;
use fake::{faker::chrono::en::Date, faker::name::en::Name, Fake};
use indicatif::{MultiProgress, ProgressBar};
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::Arc;

define_and_impl_sql_insertable!(
    T_RHSTU_PACIENTE {
        pub(crate) ID_PACIENTE: u32,
        pub(crate) NM_PACIENTE: String,
        pub(crate) NR_CPF: String,
        pub(crate) NM_RG: String,
        pub(crate) DT_NASCIMENTO: String,
        pub(crate) FL_SEXO_BIOLOGICO: String,
        pub(crate) DS_ESCOLARIDADE: String,
        pub(crate) DS_ESTADO_CIVIL: String,
        pub(crate) NM_GRUPO_SANGUINEO: String,
        pub(crate) NR_ALTURA: String,
        pub(crate) NR_PESO: String,
        pub(crate) DT_CADASTRO: String,
        pub(crate) NM_USUARIO: String
    },
    T_RHSTU_TIPO_CONTATO {
        pub(crate) ID_TIPO_CONTATO: u32,
        pub(crate) NM_TIPO_CONTATO: String,
        pub(crate) DT_INICIO: String,
        pub(crate) DT_FIM: String,
        pub(crate) DT_CADASTRO: String,
        pub(crate) NM_USUARIO: String
    },
    T_RHSTU_CONTATO_PACIENTE {
        pub(crate) ID_PACIENTE: u32,
        pub(crate) ID_CONTATO: u32,
        pub(crate) ID_TIPO_CONTATO: u32,
        pub(crate) NM_CONTATO: String,
        pub(crate) NR_DDI: String,
        pub(crate) NR_DDD: String,
        pub(crate) NR_TELEFONE: String,
        pub(crate) DT_CADASTRO: String,
        pub(crate) NM_USUARIO: String
    },
    T_RHSTU_EMAIL_PACIENTE {
        pub(crate) ID_EMAIL: u32,
        pub(crate) ID_PACIENTE: u32,
        pub(crate) DS_EMAIL: String,
        pub(crate) TP_EMAIL: String,
        pub(crate) ST_EMAIL: String,
        pub(crate) DT_CADASTRO: String,
        pub(crate) NM_USUARIO: String
    },
    T_RHSTU_TELEFONE_PACIENTE {
        pub(crate) ID_PACIENTE: u32,
        pub(crate) ID_TELEFONE: u32,
        pub(crate) NR_DDI: String,
        pub(crate) NR_DDD: String,
        pub(crate) NR_TELEFONE: u32,
        pub(crate) TP_TELEFONE: String,
        pub(crate) ST_TELEFONE: String,
        pub(crate) DT_CADASTRO: String,
        pub(crate) NM_USUARIO: String
    },
    T_RHSTU_ENDERECO_PACIENTE {
        pub(crate) ID_ENDERECO: u32,
        pub(crate) ID_PACIENTE: u32,
        pub(crate) ID_LOGRADOURO: u32,
        pub(crate) NR_LOGRADOURO: String,
        pub(crate) DS_COMPLEMENTO_NUMERO: String,
        pub(crate) DS_PONTO_REFERENCIA: String,
        pub(crate) DT_INICIO: String,
        pub(crate) DT_FIM: String,
        pub(crate) DT_CADASTRO: String,
        pub(crate) NM_USUARIO: String
    }
);

pub(crate) async fn generate_patients(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) {
    let pb_helper = ProgressBarHelper::new(m, total, "Patients:".to_string());
    let pb = &pb_helper.pb;

    let patients: Vec<T_RHSTU_PACIENTE> = (0..total)
        .into_par_iter()
        .map(|i| {
            pb.inc(1); // Increment the progress bar
            main_pb.inc(1);

            let mut rng = rand::thread_rng();
            T_RHSTU_PACIENTE {
                ID_PACIENTE: i as u32,
                NM_PACIENTE: Name().fake(),
                NR_CPF: random_cpf(),
                NM_RG: random_rg(),
                DT_NASCIMENTO: Date().fake(),
                FL_SEXO_BIOLOGICO: ["M", "F"].choose(&mut rng).unwrap().to_string(),
                DS_ESCOLARIDADE: ["Ensino Fundamental", "Ensino Médio", "Ensino Superior"]
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
                DS_ESTADO_CIVIL: ["Solteiro", "Casado", "Divorciado", "Viúvo"]
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
                NM_GRUPO_SANGUINEO: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"]
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
                DT_CADASTRO: Date().fake(),
                NM_USUARIO: Name().fake(),
                NR_ALTURA: rng.gen_range(1.0..2.0).to_string(),
                NR_PESO: rng.gen_range(50.0..100.0).to_string(),
            }
        })
        .collect();

    let generator = SqlGenerator::new(patients);
    let _ = generator.write_to_file();

    pb_helper.finish();
}

pub(crate) async fn generate_contact_types(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_TIPO_CONTATO> {
    let pb_helper = ProgressBarHelper::new(m, total, "Contact types:".to_string());
    let pb = &pb_helper.pb;

    let contact_types: Vec<T_RHSTU_TIPO_CONTATO> = (0..total)
        .into_par_iter()
        .map(|i| {
            pb.inc(1); // Increment the progress bar
            main_pb.inc(1);

            let mut rng = rand::thread_rng();
            T_RHSTU_TIPO_CONTATO {
                ID_TIPO_CONTATO: i as u32,
                // pick a random emergency contact type relationship to the patient
                NM_TIPO_CONTATO: ["Pessoal", "Trabalho", "Emergência"]
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
                DT_INICIO: Date().fake(),
                DT_FIM: Date().fake(),
                DT_CADASTRO: Date().fake(),
                NM_USUARIO: 1.to_string(),
            }
        })
        .collect();

    let generator = SqlGenerator::new(contact_types.clone());
    let _ = generator.write_to_file();

    pb_helper.finish();

    contact_types
}

pub(crate) async fn generate_patient_contacts(
    total: usize,
    contact_types: Vec<T_RHSTU_TIPO_CONTATO>,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_CONTATO_PACIENTE> {
    let pb_helper = ProgressBarHelper::new(m, total, "Patient contacts:".to_string());
    let pb = &pb_helper.pb;

    let contacts: Vec<T_RHSTU_CONTATO_PACIENTE> = (0..total)
        .into_par_iter()
        .map(|i| {
            pb.inc(1); // Increment the progress bar
            main_pb.inc(1);

            let mut rng = rand::thread_rng();
            let contact_type = contact_types.choose(&mut rng).unwrap();
            T_RHSTU_CONTATO_PACIENTE {
                ID_PACIENTE: i as u32,
                ID_CONTATO: i as u32,
                ID_TIPO_CONTATO: contact_type.ID_TIPO_CONTATO,
                NM_CONTATO: Name().fake(),
                NR_DDI: rng.gen_range(1..100).to_string(),
                NR_DDD: rng.gen_range(1..100).to_string(),
                NR_TELEFONE: rng.gen_range(1..100).to_string(),
                DT_CADASTRO: Date().fake(),
                NM_USUARIO: 1.to_string(),
            }
        })
        .collect();

    let generator = SqlGenerator::new(contacts.clone());
    let _ = generator.write_to_file();

    pb_helper.finish();

    contacts
}

pub(crate) async fn generate_emails(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_EMAIL_PACIENTE> {
    let pb_helper = ProgressBarHelper::new(m, total, "Patient emails:".to_string());
    let pb = &pb_helper.pb;

    let emails: Vec<T_RHSTU_EMAIL_PACIENTE> = (0..total)
        .into_par_iter()
        .map(|i| {
            pb.inc(1); // Increment the progress bar
            main_pb.inc(1);
            let mut rng = rand::thread_rng();

            T_RHSTU_EMAIL_PACIENTE {
                ID_EMAIL: i as u32,
                ID_PACIENTE: i as u32,
                DS_EMAIL: FreeEmail().fake(),
                TP_EMAIL: ["Pessoal", "Trabalho"]
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
                ST_EMAIL: ["Ativo", "Inativo"].choose(&mut rng).unwrap().to_string(),
                DT_CADASTRO: Date().fake(),
                NM_USUARIO: 1.to_string(),
            }
        })
        .collect();

    let generator = SqlGenerator::new(emails.clone());
    let _ = generator.write_to_file();

    pb_helper.finish();

    emails
}

pub(crate) async fn generate_telephones(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_TELEFONE_PACIENTE> {
    let pb_helper = ProgressBarHelper::new(m, total, "Patient telephones:".to_string());
    let pb = &pb_helper.pb;

    let ddds = get_ddds().unwrap();

    let telephones: Vec<T_RHSTU_TELEFONE_PACIENTE> = (0..total)
        .into_par_iter()
        .map(|i| {
            pb.inc(1); // Increment the progress bar
            main_pb.inc(1);

            let mut rng = rand::thread_rng();

            T_RHSTU_TELEFONE_PACIENTE {
                ID_PACIENTE: i as u32,
                ID_TELEFONE: i as u32,
                NR_DDI: rng.gen_range(1..100).to_string(),
                NR_DDD: ddds.choose(&mut rng).unwrap().to_string(),
                NR_TELEFONE: random_br_phone(),
                TP_TELEFONE: ["Pessoal", "Trabalho"]
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
                ST_TELEFONE: ["Ativo", "Inativo"].choose(&mut rng).unwrap().to_string(),
                DT_CADASTRO: Date().fake(),
                NM_USUARIO: 1.to_string(),
            }
        })
        .collect();

    let generator = SqlGenerator::new(telephones.clone());
    let _ = generator.write_to_file();

    pb_helper.finish();

    telephones
}

pub(crate) async fn generate_patients_addresses(
    patients: usize,
    address: Vec<TRHSTU_LOGRADOURO>,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_ENDERECO_PACIENTE> {
    let pb_helper = ProgressBarHelper::new(m, patients, "Patient addresses:".to_string());
    let pb = &pb_helper.pb;

    let patient_addresses: Vec<T_RHSTU_ENDERECO_PACIENTE> = (0..patients)
        .into_par_iter()
        .map(|i| {
            pb.inc(1); // Increment the progress bar
            main_pb.inc(1);

            let mut rng = rand::thread_rng();
            let chosen_address = address.choose(&mut rng).unwrap().clone();

            T_RHSTU_ENDERECO_PACIENTE {
                ID_ENDERECO: chosen_address.ID_LOGRADOURO,
                ID_PACIENTE: i as u32,
                ID_LOGRADOURO: chosen_address.ID_LOGRADOURO,
                NR_LOGRADOURO: rng.gen_range(1..100).to_string(),
                DS_COMPLEMENTO_NUMERO: "DS_COMPLEMENTO_NUMERO".to_string(),
                DS_PONTO_REFERENCIA: "DS_PONTO_REFERENCIA".to_string(),
                DT_INICIO: Date().fake(),
                DT_FIM: Date().fake(),
                DT_CADASTRO: Date().fake(),
                NM_USUARIO: 1.to_string(),
            }
        })
        .collect();

    let generator = SqlGenerator::new(patient_addresses.clone());
    let _ = generator.write_to_file();

    pb_helper.finish();

    patient_addresses
}

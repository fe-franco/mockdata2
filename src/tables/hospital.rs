use crate::common::{current_timestamp, random_cpf, random_rg, ProgressBarHelper};
use crate::define_and_impl_sql_insertable;
use crate::sql_generator::SqlGenerator;
use fake::{
    faker::{
        address::en::{BuildingNumber, SecondaryAddress, StreetName},
        company::en::CompanyName,
        name::en::Name,
    },
    Fake,
};
use indicatif::{MultiProgress, ProgressBar};
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};

use super::geography::T_RHSTU_LOGRADOURO;

define_and_impl_sql_insertable!(
    T_RHSTU_UNID_HOSPITALAR {
        ID_UNID_HOSPITAL: u64,
        NM_UNID_HOSPITALAR: String,
        NM_RAZAO_SOCIAL_UNID_HOSP: String,
        DT_FUNDACAO: String,
        NR_LOGRADOURO: u64,
        DS_COMPLEMENTO_NUMERO: String,
        DS_PONTO_REFERENCIA: String,
        DT_INICIO: String,
        DT_TERMINO: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_ENDERECO_UNIDHOSP {
        ID_END_UNIDHOSP: u64,
        ID_UNID_HOSPITAL: u64,
        ID_LOGRADOURO: u64,
        NR_LOGRADOURO: u64,
        DS_COMPLEMENTO_NUMERO: String,
        DS_PONTO_REFERENCIA: String,
        DT_INICIO: String,
        DT_FIM: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_FUNCIONARIO {
        ID_FUNC: u64,
        ID_SUPERIOR: u64,
        NM_FUNC: String,
        DS_CARGO: String,
        DT_NASCIMENTO: String,
        VL_SALARIO: u64,
        NR_RG: String,
        NR_CPF: u64,
        ST_FUNC: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_MEDICO {
        ID_FUNC: u64,
        NR_CRM: u64,
        DS_ESPECIALIDADE: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_MOTORISTA {
        ID_FUNC: u64,
        NR_CNH: u64,
        NM_CATEGORIA_CNH: String,
        DT_VALIDADE_CNH: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    }
);

pub(crate) async fn generate_hospital(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> usize {
    let mut len = 0;

    let pb_helper = ProgressBarHelper::new(m, total * 2, "Hospitals:".to_string());
    let pb = &pb_helper.pb;

    let mut hospitals: Vec<T_RHSTU_UNID_HOSPITALAR> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..total {
        let hospital = T_RHSTU_UNID_HOSPITALAR {
            ID_UNID_HOSPITAL: i as u64,
            NM_UNID_HOSPITALAR: CompanyName().fake(),
            NM_RAZAO_SOCIAL_UNID_HOSP: CompanyName().fake(),
            DT_FUNDACAO: current_timestamp(),
            NR_LOGRADOURO: rng.gen_range(1..1000) as u64,
            DS_COMPLEMENTO_NUMERO: SecondaryAddress().fake(),
            DS_PONTO_REFERENCIA: SecondaryAddress().fake(),
            DT_INICIO: current_timestamp(),
            DT_TERMINO: current_timestamp(),
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: Name().fake(),
        };

        hospitals.push(hospital);

        len += 1;
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(hospitals);
    let _ = generator.write_to_file(pb);

    pb_helper.finish();

    len
}

pub(crate) async fn generate_hospital_address(
    total: usize,
    address: Vec<T_RHSTU_LOGRADOURO>,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> usize {
    let mut len = 0;

    let mut rng = rand::thread_rng();

    let pb_helper = ProgressBarHelper::new(m, total * 2, "T_RHSTU_ENDERECO_UNIDHOSP:".to_string());
    let pb = &pb_helper.pb;

    let mut hospitals_addresses: Vec<T_RHSTU_ENDERECO_UNIDHOSP> = Vec::new();

    for i in 0..total {
        let hospital_address = T_RHSTU_ENDERECO_UNIDHOSP {
            ID_UNID_HOSPITAL: i as u64,
            ID_END_UNIDHOSP: i as u64,
            ID_LOGRADOURO: address[i].ID_LOGRADOURO,
            NR_LOGRADOURO: rng.gen_range(1..1000) as u64,
            DS_COMPLEMENTO_NUMERO: BuildingNumber().fake(),
            DS_PONTO_REFERENCIA: BuildingNumber().fake(),
            DT_INICIO: current_timestamp(),
            DT_FIM: current_timestamp(),
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: Name().fake(),
        };

        hospitals_addresses.push(hospital_address);

        len += 1;
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(hospitals_addresses);
    let _ = generator.write_to_file(pb);

    pb_helper.finish();

    len
}

pub(crate) async fn generate_employee(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<u64> {
    let pb_helper = ProgressBarHelper::new(m, total * 2, "Employees:".to_string());
    let pb = &pb_helper.pb;

    let employees: Vec<T_RHSTU_FUNCIONARIO> = (0..total)
        .into_par_iter()
        .map(|i| {
            let mut rng = rand::thread_rng();
            let employee = T_RHSTU_FUNCIONARIO {
                ID_FUNC: i as u64,
                ID_SUPERIOR: i as u64,
                NM_FUNC: Name().fake(),
                DS_CARGO: Name().fake(),
                DT_NASCIMENTO: current_timestamp(),
                VL_SALARIO: rng.gen_range(1000.0..10000.0) as u64,
                NR_RG: random_rg().to_string(),
                NR_CPF: random_cpf(),
                ST_FUNC: ["A", "I"].choose(&mut rng).unwrap().to_string(),
                DT_CADASTRO: current_timestamp(),
                NM_USUARIO: Name().fake(),
            };

            pb.inc(1);
            main_pb.inc(1);

            employee
        })
        .collect();

    let generator = SqlGenerator::new(employees.clone());
    let _ = generator.write_to_file(pb);

    pb_helper.finish();

    employees.into_par_iter().map(|e| e.ID_FUNC).collect()
}

pub(crate) async fn generate_doctor(
    employee_ids: Arc<Mutex<Vec<u64>>>,
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> usize {
    let mut employee_ids_guard = employee_ids.lock().unwrap();

    if total > employee_ids_guard.len() {
        panic!("Not enough employees to generate doctors");
    }

    let pb_helper = ProgressBarHelper::new(m, total * 2, "Doctors:".to_string());
    let pb = &pb_helper.pb;

    let mut rng = rand::thread_rng(); // Reuse the random number generator
    let mut doctors = Vec::new(); // For example, a batch size of 1000

    for i in 0..total {
        let doctor = T_RHSTU_MEDICO {
            ID_FUNC: employee_ids_guard[i],
            NR_CRM: rng.gen_range(1000000..9999999) as u64,
            DS_ESPECIALIDADE: Name().fake(),
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: Name().fake(),
        };

        doctors.push(doctor);

        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(doctors);
    let _ = generator.write_to_file(pb);

    // Remove used employee IDs after the loop to avoid shifting elements multiple times
    employee_ids_guard.drain(0..total);

    pb_helper.finish();

    total
}

pub(crate) async fn generate_driver(
    employee_ids: Arc<Mutex<Vec<u64>>>,
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> usize {
    let mut employee_ids_guard = employee_ids.lock().unwrap();

    if total > employee_ids_guard.len() {
        panic!("Not enough employees to generate drivers");
    }

    let pb_helper = ProgressBarHelper::new(m, total * 2, "Drivers:".to_string());
    let pb = &pb_helper.pb;

    let mut rng = rand::thread_rng(); // Reuse the random number generator
    let mut drivers = Vec::with_capacity(1000); // For example, a batch size of 1000

    for i in 0..total {
        let driver = T_RHSTU_MOTORISTA {
            ID_FUNC: employee_ids_guard[i],
            NR_CNH: rng.gen_range(1000000..9999999) as u64,
            NM_CATEGORIA_CNH: ["A", "B", "C", "D", "E"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            DT_VALIDADE_CNH: current_timestamp(),
            DT_CADASTRO: current_timestamp(),
            NM_USUARIO: Name().fake(),
        };

        drivers.push(driver);

        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(drivers);
    let _ = generator.write_to_file(pb);

    // Remove used employee IDs after the loop to avoid shifting elements multiple times
    employee_ids_guard.drain(0..total);

    pb_helper.finish();

    total
}

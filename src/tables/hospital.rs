use crate::common::{random_cep, random_cpf, random_rg, ProgressBarHelper};
use crate::define_and_impl_sql_insertable;
use crate::sql_generator::SqlGenerator;
use crate::tables::geography::{TRHSTU_BAIRRO, TRHSTU_CIDADE};
use fake::{
    faker::{
        address::en::{BuildingNumber, StreetName},
        chrono::en::Date,
        company::en::CompanyName,
        name::en::Name,
    },
    Fake,
};
use indicatif::{MultiProgress, ProgressBar};
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};

define_and_impl_sql_insertable!(
    T_RHSTU_UNID_HOSPITALAR {
        ID_UNID_HOSPITAL: u32,
        NM_UNID_HOSPITALAR: String,
        NM_RAZAO_SOCIAL_UNID_HOSP: String,
        DT_FUNDACAO: String,
        NR_LOGRADOURO: String,
        DS_COMPLEMENTO_NUMERO: String,
        DS_PONTO_REFERENCIA: String,
        DT_INICIO: String,
        DT_TERMINO: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_ENDERECO_UNIDHOSP {
        ID_UNID_HOSPITAL: u32,
        ID_BAIRRO: u32,
        ID_CIDADE: u32,
        ID_ESTADO: u32,
        NR_CEP: String,
        NR_DDD: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_FUNCIONARIO {
        ID_FUNC: u32,
        ID_SUPERIOR: u32,
        NM_FUNC: String,
        DS_CARGO: String,
        DT_NASCIMENTO: String,
        VL_SALARIO: String,
        NR_RG: String,
        NR_CPF: String,
        ST_FUNC: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_MEDICO_ROWS {
        ID_FUNC: u32,
        NR_CRM: String,
        DS_ESPECIALIDADE: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_MOTORISTA_ROWS {
        ID_FUNC: u32,
        NR_CNH: String,
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

    let pb_helper = ProgressBarHelper::new(m, total, "Hospitals:".to_string());
    let pb = &pb_helper.pb;

    let mut hospitals: Vec<T_RHSTU_UNID_HOSPITALAR> = Vec::new();

    for i in 0..total {
        let hospital = T_RHSTU_UNID_HOSPITALAR {
            ID_UNID_HOSPITAL: i as u32,
            NM_UNID_HOSPITALAR: CompanyName().fake(),
            NM_RAZAO_SOCIAL_UNID_HOSP: CompanyName().fake(),
            DT_FUNDACAO: Date().fake(),
            NR_LOGRADOURO: StreetName().fake(),
            DS_COMPLEMENTO_NUMERO: BuildingNumber().fake(),
            DS_PONTO_REFERENCIA: BuildingNumber().fake(),
            DT_INICIO: Date().fake(),
            DT_TERMINO: Date().fake(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        hospitals.push(hospital);

        len += 1;
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(hospitals);
    generator.write_to_file();

    pb_helper.finish();

    len
}

pub(crate) async fn generate_hospital_address(
    total: usize,
    trhstu_bairros: Vec<TRHSTU_BAIRRO>,
    cities: Vec<TRHSTU_CIDADE>,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> usize {
    let mut len = 0;

    let mut rng = rand::thread_rng();

    let pb_helper = ProgressBarHelper::new(
        m,
        total,
        "T_RHSTU_UNID_HOSPITALAR_ROWS Addresses:".to_string(),
    );
    let pb = &pb_helper.pb;

    let mut hospitals_addresses: Vec<T_RHSTU_ENDERECO_UNIDHOSP> = Vec::new();

    for i in 0..total {
        let trhstu_bairro = trhstu_bairros.choose(&mut rng).unwrap().clone();

        // find TRHSTU_CIDADE and state ids from TRHSTU_BAIRRO
        let trhstu_cidade = cities
            .iter()
            .find(|&c| c.ID_CIDADE == trhstu_bairro.ID_CIDADE)
            .unwrap();

        let hospital_address = T_RHSTU_ENDERECO_UNIDHOSP {
            ID_UNID_HOSPITAL: i as u32,
            ID_BAIRRO: trhstu_bairro.ID_BAIRRO,
            ID_CIDADE: trhstu_cidade.ID_CIDADE,
            ID_ESTADO: trhstu_cidade.ID_ESTADO,
            NR_CEP: random_cep(),
            NR_DDD: trhstu_cidade.NR_DDD.clone(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        hospitals_addresses.push(hospital_address);

        len += 1;
        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(hospitals_addresses);
    generator.write_to_file();

    pb_helper.finish();

    len
}

pub(crate) async fn generate_employee(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<u32> {
    let mut writer = csv::Writer::from_path("data/employee.csv").unwrap();

    let pb_helper = ProgressBarHelper::new(m, total, "Employees:".to_string());
    let pb = &pb_helper.pb;

    let employees: Vec<T_RHSTU_FUNCIONARIO> = (0..total)
        .into_par_iter()
        .map(|i| {
            let mut rng = rand::thread_rng();
            let employee = T_RHSTU_FUNCIONARIO {
                ID_FUNC: i as u32,
                ID_SUPERIOR: i as u32,
                NM_FUNC: Name().fake(),
                DS_CARGO: Name().fake(),
                DT_NASCIMENTO: Date().fake(),
                VL_SALARIO: rng.gen_range(1000.0..10000.0).to_string(),
                NR_RG: random_rg(),
                NR_CPF: random_cpf(),
                ST_FUNC: ["A", "I"].choose(&mut rng).unwrap().to_string(),
                DT_CADASTRO: Date().fake(),
                NM_USUARIO: Name().fake(),
            };

            pb.inc(1);
            main_pb.inc(1);

            employee
        })
        .collect();

    let generator = SqlGenerator::new(employees.clone());
    generator.write_to_file();

    pb_helper.finish();

    employees.into_par_iter().map(|e| e.ID_FUNC).collect()
}

pub(crate) async fn generate_doctor(
    employee_ids: Arc<Mutex<Vec<u32>>>,
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> usize {
    let mut employee_ids_guard = employee_ids.lock().unwrap();

    if total > employee_ids_guard.len() {
        panic!("Not enough employees to generate doctors");
    }

    let pb_helper = ProgressBarHelper::new(m, total, "Doctors:".to_string());
    let pb = &pb_helper.pb;

    let mut rng = rand::thread_rng(); // Reuse the random number generator
    let mut doctors = Vec::new(); // For example, a batch size of 1000

    for i in 0..total {
        let doctor = T_RHSTU_MEDICO_ROWS {
            ID_FUNC: employee_ids_guard[i],
            NR_CRM: rng.gen_range(1000000..9999999).to_string(),
            DS_ESPECIALIDADE: Name().fake(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        doctors.push(doctor);

        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(doctors);
    generator.write_to_file();

    // Remove used employee IDs after the loop to avoid shifting elements multiple times
    employee_ids_guard.drain(0..total);

    pb_helper.finish();

    total
}

pub(crate) async fn generate_driver(
    employee_ids: Arc<Mutex<Vec<u32>>>,
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> usize {
    let mut employee_ids_guard = employee_ids.lock().unwrap();

    if total > employee_ids_guard.len() {
        panic!("Not enough employees to generate drivers");
    }

    let mut writer = csv::Writer::from_path("data/driver.csv").unwrap();
    let pb_helper = ProgressBarHelper::new(m, total, "Drivers:".to_string());
    let pb = &pb_helper.pb;

    let mut rng = rand::thread_rng(); // Reuse the random number generator
    let mut drivers = Vec::with_capacity(1000); // For example, a batch size of 1000

    for i in 0..total {
        let driver = T_RHSTU_MOTORISTA_ROWS {
            ID_FUNC: employee_ids_guard[i],
            NR_CNH: rng.gen_range(1000000..9999999).to_string(),
            NM_CATEGORIA_CNH: ["A", "B", "C", "D", "E"]
                .choose(&mut rng)
                .unwrap()
                .to_string(),
            DT_VALIDADE_CNH: Date().fake(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        drivers.push(driver);

        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(drivers);
    generator.write_to_file();

    // Remove used employee IDs after the loop to avoid shifting elements multiple times
    employee_ids_guard.drain(0..total);

    pb_helper.finish();

    total
}

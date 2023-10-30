use fake::{
    faker::{
        address::en::{BuildingNumber, StreetName},
        chrono::en::Date,
        company::en::CompanyName,
        name::en::Name,
    },
    Fake,
};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    common::{random_cep, random_cpf, random_rg},
    geography::{City, Neighborhood},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(crate) struct Hospital {
    pub(crate) ID_UNID_HOSPITAL: u32,
    pub(crate) NM_UNID_HOSPITALAR: String,
    pub(crate) NM_RAZAO_SOCIAL_UNID_HOSP: String,
    pub(crate) DT_FUNDACAO: String,
    pub(crate) NR_LOGRADOURO: String,
    pub(crate) DS_COMPLEMENTO_NUMERO: String,
    pub(crate) DS_PONTO_REFERENCIA: String,
    pub(crate) DT_INICIO: String,
    pub(crate) DT_TERMINO: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(crate) struct HospitalAddress {
    pub(crate) ID_UNID_HOSPITAL: u32,
    pub(crate) ID_BAIRRO: u32,
    pub(crate) ID_CIDADE: u32,
    pub(crate) ID_ESTADO: u32,
    pub(crate) NR_CEP: String,
    pub(crate) NR_DDD: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]

struct Employee {
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
    NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]

struct Doctor {
    ID_FUNC: u32,
    NR_CRM: String,
    DS_ESPECIALIDADE: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]

struct Driver {
    ID_FUNC: u32,
    NR_CNH: String,
    NM_CATEGORIA_CNH: String,
    DT_VALIDADE_CNH: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

pub(crate) async fn generate_hospital(total: usize) -> usize {
    let mut len = 0;

    let mut writer = csv::Writer::from_path("data/hospital.csv").unwrap();

    for i in 0..total {
        let hospital = Hospital {
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

        writer.serialize(hospital).unwrap();
        len += 1;
    }

    len
}

pub(crate) async fn generate_hospital_address(
    total: usize,
    neighborhoods: Vec<Neighborhood>,
    cities: Vec<City>,
) -> usize {
    let mut len = 0;

    let mut writer = csv::Writer::from_path("data/hospital_address.csv").unwrap();
    let mut rng = rand::thread_rng();
    for i in 0..total {
        let neighborhood = neighborhoods.choose(&mut rng).unwrap().clone();

        // find city and state ids from neighborhood
        let city = cities
            .iter()
            .find(|&c| c.ID_CIDADE == neighborhood.ID_CIDADE)
            .unwrap();

        let hospital_address = HospitalAddress {
            ID_UNID_HOSPITAL: i as u32,
            ID_BAIRRO: neighborhood.ID_BAIRRO,
            ID_CIDADE: city.ID_CIDADE,
            ID_ESTADO: city.ID_ESTADO,
            NR_CEP: random_cep(),
            NR_DDD: city.NR_DDD.clone(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(hospital_address).unwrap();

        len += 1;
    }

    len
}

pub(crate) async fn generate_employee(total: usize) -> Vec<u32> {
    let mut employee_ids: Vec<u32> = Vec::new();

    let mut writer = csv::Writer::from_path("data/employee.csv").unwrap();

    for i in 0..total {
        let employee = Employee {
            ID_FUNC: i as u32,
            ID_SUPERIOR: i as u32,
            NM_FUNC: Name().fake(),
            DS_CARGO: Name().fake(),
            DT_NASCIMENTO: Date().fake(),
            VL_SALARIO: rand::thread_rng().gen_range(1000.0..10000.0).to_string(),
            NR_RG: random_rg(),
            NR_CPF: random_cpf(),
            ST_FUNC: ["A", "I"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(employee).unwrap();

        employee_ids.push(i as u32);
    }

    employee_ids
}

pub(crate) fn generate_doctor(employee_ids: &mut Vec<u32>, total: usize) -> usize {
    if total > employee_ids.len() {
        panic!("Not enough employees to generate doctors")
    }

    let mut len = 0;

    let mut writer = csv::Writer::from_path("data/doctor.csv").unwrap();

    for i in 0..total {
        let doctor = Doctor {
            ID_FUNC: employee_ids[i],
            NR_CRM: rand::thread_rng().gen_range(1000000..9999999).to_string(),
            DS_ESPECIALIDADE: Name().fake(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(doctor).unwrap();

        employee_ids.remove(i);
        len += 1;
    }

    len
}

pub(crate) fn generate_driver(employee_ids: &mut Vec<u32>, total: usize) -> usize {
    if total > employee_ids.len() {
        panic!("Not enough employees to generate drivers")
    }

    let mut len = 0;

    let mut writer = csv::Writer::from_path("data/driver.csv").unwrap();

    for i in 0..total {
        let driver = Driver {
            ID_FUNC: employee_ids[i],
            NR_CNH: rand::thread_rng().gen_range(1000000..9999999).to_string(),
            NM_CATEGORIA_CNH: ["A", "B", "C", "D", "E"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DT_VALIDADE_CNH: Date().fake(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(driver).unwrap();

        employee_ids.remove(i);
        len += 1;
    }

    len
}

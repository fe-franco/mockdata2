use fake::{faker::chrono::en::Date, faker::name::en::Name, Fake};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use crate::common::{random_cpf, random_rg};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Patient {
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
    pub(crate) NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ContactType {
    pub(crate) ID_TIPO_CONTATO: u32,
    pub(crate) NM_TIPO_CONTATO: String,
    pub(crate) DT_INICIO: String,
    pub(crate) DT_FIM: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Contact {
    pub(crate) ID_PACIENTE: u32,
    pub(crate) ID_CONTATO: u32,
    pub(crate) ID_TIPO_CONTATO: u32,
    pub(crate) NM_CONTATO: String,
    pub(crate) NR_DDI: String,
    pub(crate) NR_DDD: String,
    pub(crate) NR_TELEFONE: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Email {
    pub(crate) ID_EMAIL: u32,
    pub(crate) ID_PACIENTE: u32,
    pub(crate) DS_EMAIL: String,
    pub(crate) TP_EMAIL: String,
    pub(crate) ST_EMAIL: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Telephone {
    pub(crate) ID_PACIENTE: u32,
    pub(crate) ID_TELEFONE: u32,
    pub(crate) NR_DDI: String,
    pub(crate) NR_DDD: String,
    pub(crate) NR_TELEFONE: String,
    pub(crate) TP_TELEFONE: String,
    pub(crate) ST_TELEFONE: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Address {
    pub(crate) ID_ENDERECO: u32,
    pub(crate) ID_PACIENTE: u32,
    pub(crate) ID_LOGRADOURO: u32,
    pub(crate) NR_LOGRADOURO: String,
    pub(crate) DS_COMPLEMENTO_NUMERO: String,
    pub(crate) DS_PONTO_REFERENCIA: String,
    pub(crate) DT_INICIO: String,
    pub(crate) DT_FIM: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

pub(crate) fn generate_patients(total: usize) -> Vec<Patient> {
    let mut patients: Vec<Patient> = Vec::new();

    let mut writer = csv::Writer::from_path("data/patient.csv").unwrap();

    for i in 0..total {
        let patient = Patient {
            ID_PACIENTE: i as u32,
            NM_PACIENTE: Name().fake(),
            NR_CPF: random_cpf(),
            NM_RG: random_rg(),
            // get ramdom date
            DT_NASCIMENTO: Date().fake(),
            FL_SEXO_BIOLOGICO: ["M", "F"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DS_ESCOLARIDADE: ["Ensino Fundamental", "Ensino Médio", "Ensino Superior"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DS_ESTADO_CIVIL: ["Solteiro", "Casado", "Divorciado", "Viúvo"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            NM_GRUPO_SANGUINEO: ["A+", "A-", "B+", "B-", "AB+", "AB-", "O+", "O-"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
            NR_ALTURA: rand::thread_rng().gen_range(1.0..2.0).to_string(),
            NR_PESO: rand::thread_rng().gen_range(50.0..100.0).to_string(),
        };

        writer.serialize(&patient).unwrap();

        patients.push(patient);
    }
    patients
}

pub(crate) fn generate_contact_type(total: usize) -> Vec<ContactType> {
    let mut contact_types: Vec<ContactType> = Vec::new();

    let mut writer = csv::Writer::from_path("data/contact_type.csv").unwrap();

    for i in 0..total {
        let contact_type = ContactType {
            ID_TIPO_CONTATO: i as u32,
            NM_TIPO_CONTATO: ["Telefone", "Celular", "Email"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DT_INICIO: Date().fake(),
            DT_FIM: Date().fake(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(&contact_type).unwrap();

        contact_types.push(contact_type);
    }
    contact_types
}

pub(crate) fn generate_contact(total: usize) -> Vec<Contact> {
    let mut contacts: Vec<Contact> = Vec::new();

    let mut writer = csv::Writer::from_path("data/contact.csv").unwrap();

    for i in 0..total {
        let contact = Contact {
            ID_PACIENTE: i as u32,
            ID_CONTATO: i as u32,
            ID_TIPO_CONTATO: i as u32,
            NM_CONTATO: Name().fake(),
            NR_DDI: rand::thread_rng().gen_range(1..100).to_string(),
            NR_DDD: rand::thread_rng().gen_range(1..100).to_string(),
            NR_TELEFONE: rand::thread_rng().gen_range(1..100).to_string(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(&contact).unwrap();

        contacts.push(contact);
    }
    contacts
}

pub(crate) fn generate_email(total: usize) -> Vec<Email> {
    let mut emails: Vec<Email> = Vec::new();

    let mut writer = csv::Writer::from_path("data/email.csv").unwrap();

    for i in 0..total {
        let email = Email {
            ID_EMAIL: i as u32,
            ID_PACIENTE: i as u32,
            DS_EMAIL: Name().fake(),
            TP_EMAIL: ["Pessoal", "Trabalho"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            ST_EMAIL: ["Ativo", "Inativo"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(&email).unwrap();

        emails.push(email);
    }
    emails
}

pub(crate) fn generate_telephone(total: usize) -> Vec<Telephone> {
    let mut telephones: Vec<Telephone> = Vec::new();

    let mut writer = csv::Writer::from_path("data/telephone.csv").unwrap();

    for i in 0..total {
        let telephone = Telephone {
            ID_PACIENTE: i as u32,
            ID_TELEFONE: i as u32,
            NR_DDI: rand::thread_rng().gen_range(1..100).to_string(),
            NR_DDD: rand::thread_rng().gen_range(1..100).to_string(),
            NR_TELEFONE: rand::thread_rng().gen_range(1..100).to_string(),
            TP_TELEFONE: ["Pessoal", "Trabalho"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            ST_TELEFONE: ["Ativo", "Inativo"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(&telephone).unwrap();

        telephones.push(telephone);
    }
    telephones
}

pub(crate) fn generate_patient_address(total: usize) -> Vec<Address> {
    let mut addresses: Vec<Address> = Vec::new();

    let mut writer = csv::Writer::from_path("data/patient_address.csv").unwrap();
    // choose between 1 and 16266 for the ID_LOGRADOURO
    // choose between 1 and 100 for the NR_LOGRADOURO

    for i in 0..total {
        let address = Address {
            ID_ENDERECO: i as u32,
            ID_PACIENTE: i as u32,
            ID_LOGRADOURO: rand::thread_rng().gen_range(1..16266),
            NR_LOGRADOURO: rand::thread_rng().gen_range(1..1000).to_string(),
            DS_COMPLEMENTO_NUMERO: Name().fake(),
            DS_PONTO_REFERENCIA: Name().fake(),
            DT_INICIO: Date().fake(),
            DT_FIM: Date().fake(),
            DT_CADASTRO: Date().fake(),
            NM_USUARIO: Name().fake(),
        };

        writer.serialize(&address).unwrap();

        addresses.push(address);
    }
    addresses
}

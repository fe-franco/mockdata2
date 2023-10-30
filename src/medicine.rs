// use std::io::{self, Write};

use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::task;

use crate::common::{fetch_data, fetch_with_exponential_backoff};

// - T_RHSTU_MEDICAMENTO - "ID_MEDICAMENTO","NM_MEDICAMENTO","DS_DETALHADA_MEDICAMENTO","NR_CODIGO_BARRAS","DT_CADASTRO","NM_USUARIO"
#[derive(Deserialize, Debug, Serialize, Clone)]
#[allow(non_snake_case)]
pub(crate) struct Medicine {
    pub(crate) ID_MEDICAMENTO: u32,
    pub(crate) NM_MEDICAMENTO: String,
    pub(crate) DS_DETALHADA_MEDICAMENTO: String,
    pub(crate) NR_CODIGO_BARRAS: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

// - T_RHSTU_PRESCRICAO_MEDICA - "ID_PRESCRICAO_MEDICA","ID_UNID_HOSPITAL","ID_CONSULTA","ID_MEDICAMENTO","DS_POSOLOGIA","DS_VIA","DS_OBSERVACAO_USO","QT_MEDICAMENTO","NM_USUARIO","DT_CADASTRO"
#[derive(Deserialize, Debug, Serialize)]
#[allow(non_snake_case)]
pub(crate) struct MedicalPrescription {
    pub(crate) ID_PRESCRICAO_MEDICA: u32,
    pub(crate) ID_UNID_HOSPITAL: u32,
    pub(crate) ID_CONSULTA: u32,
    pub(crate) ID_MEDICAMENTO: u32,
    pub(crate) DS_POSOLOGIA: String,
    pub(crate) DS_VIA: String,
    pub(crate) DS_OBSERVACAO_USO: String,
    pub(crate) QT_MEDICAMENTO: String,
    pub(crate) NM_USUARIO: String,
    pub(crate) DT_CADASTRO: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct MedicineCategories {
    id: u32,
    descricao: String,
    ativo: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct MedicineCategoriesApi {
    categorias: Vec<MedicineCategories>,
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(non_snake_case)]
struct MedicineApi {
    content: Vec<MedicineApiContent>,
    totalElements: usize,
    totalPages: usize,
    last: bool,
    numberOfElements: usize,
    first: bool,
    sort: Option<String>,
    size: usize,
    number: usize,
}

#[derive(Deserialize, Debug, Serialize)]
#[allow(non_snake_case)]
struct MedicineApiContent {
    idProduto: u32,
    numeroRegistro: String,
    nomeProduto: String,
    expediente: String,
    razaoSocial: String,
    cnpj: String,
    numeroTransacao: String,
    data: String,
    numProcesso: String,
    idBulaPacienteProtegido: String,
    idBulaProfissionalProtegido: String,
    dataAtualizacao: String,
}

//  GET https://bula.vercel.app/categorias
async fn get_medicine_categories() -> Vec<usize> {
    let client = reqwest::Client::new();
    let response: MedicineCategoriesApi = fetch_data(&client, "https://bula.vercel.app/categorias")
        .await
        .unwrap();

    response
        .categorias
        .iter()
        .map(|category| category.id as usize)
        .collect()
}

//  GET https://bula.vercel.app/medicamentos?categoria=9&pagina=1000
pub(crate) async fn get_medicines() -> Vec<Medicine> {
    let category_ids = get_medicine_categories().await;
    let client = reqwest::Client::new();

    // Shared data structure for all tasks to aggregate their results
    let aggregated_results = Arc::new(Mutex::new(Vec::new()));

    // Spawn a new task for each category
    let tasks: Vec<_> = category_ids
        .into_iter()
        .map(|category_id| {
            let client = client.clone();
            let shared_results = aggregated_results.clone();
            task::spawn(async move {
                let results = process_category(&client, category_id).await;
                let mut shared = shared_results.lock().unwrap();
                shared.extend(results);
            })
        })
        .collect();

    // Wait for all tasks to complete
    futures::future::join_all(tasks).await;

    // Use a block to ensure the mutex guard is dropped before the end of the function
    {
        let mut writer = csv::Writer::from_path("data/medicine.csv").unwrap();
        let locked_results = aggregated_results.lock().unwrap();
        for medicine_data in locked_results.iter() {
            writer.serialize(medicine_data).unwrap();
        }
    } // Mutex guard and writer are dropped here

    let locked_results = aggregated_results.lock().unwrap();

    let mut medicines: Vec<Medicine> = Vec::new();

    for medicine_data in locked_results.iter() {
        medicines.push(medicine_data.clone());
    }

    medicines
}

async fn process_category(client: &reqwest::Client, category_id: usize) -> Vec<Medicine> {
    // println!("\nCategory: {}", category_id);
    let url_first = format!(
        "https://bula.vercel.app/medicamentos?categoria={}",
        category_id
    );

    let body_first: MedicineApi = fetch_data(&client, &url_first).await.unwrap();
    let mut medicines = Vec::new();

    for page in 1..body_first.totalPages {
        // print!("\r{}/{} ", page, body_first.totalPages);
        // io::stdout().flush().unwrap();

        let url = format!(
            "https://bula.vercel.app/medicamentos?categoria={}&pagina={}",
            category_id, page
        );

        let result: MedicineApi = match fetch_with_exponential_backoff(&client, &url).await {
            Ok(result) => result,
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
        };

        for medicine in result.content {
            let medicine_data = Medicine {
                ID_MEDICAMENTO: medicine.idProduto,
                NM_MEDICAMENTO: medicine.nomeProduto,
                DS_DETALHADA_MEDICAMENTO: medicine.expediente,
                NR_CODIGO_BARRAS: medicine.numeroRegistro,
                DT_CADASTRO: chrono::Local::now().to_string(),
                NM_USUARIO: "1".to_string(),
            };
            medicines.push(medicine_data);
        }
    }

    medicines
}

// MedicalPrescription
// - T_RHSTU_PRESCRICAO_MEDICA - "ID_PRESCRICAO_MEDICA","ID_UNID_HOSPITAL","ID_CONSULTA","ID_MEDICAMENTO","DS_POSOLOGIA","DS_VIA","DS_OBSERVACAO_USO","QT_MEDICAMENTO","NM_USUARIO","DT_CADASTRO"
pub(crate) fn generate_medical_prescription(total: usize, medicines: Vec<Medicine>) {
    let mut writer = csv::Writer::from_path("data/medical_prescription.csv").unwrap();

    for i in 0..total {
        let medicine = medicines.choose(&mut rand::thread_rng()).unwrap();

        let medical_prescription = MedicalPrescription {
            ID_PRESCRICAO_MEDICA: i as u32,
            ID_UNID_HOSPITAL: rand::thread_rng().gen_range(1..100) as u32,
            ID_CONSULTA: rand::thread_rng().gen_range(1..100) as u32,
            ID_MEDICAMENTO: medicine.ID_MEDICAMENTO,
            DS_POSOLOGIA: "DS_POSOLOGIA".to_string(),
            DS_VIA: "DS_VIA".to_string(),
            DS_OBSERVACAO_USO: "DS_OBSERVACAO_USO".to_string(),
            QT_MEDICAMENTO: "QT_MEDICAMENTO".to_string(),
            NM_USUARIO: "NM_USUARIO".to_string(),
            DT_CADASTRO: chrono::Local::now().to_string(),
        };

        writer.serialize(&medical_prescription).unwrap();
    }
}

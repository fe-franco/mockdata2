// use std::io::{self, Write};


use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::task;

use crate::{bulario::BularioClient, common::{ProgressBarHelper, current_timestamp, StringUtils}, define_and_impl_sql_insertable, sql_generator::SqlGenerator};

// - T_RHSTU_MEDICAMENTO - "ID_MEDICAMENTO","NM_MEDICAMENTO","DS_DETALHADA_MEDICAMENTO","NR_CODIGO_BARRAS","DT_CADASTRO","NM_USUARIO"
// - T_RHSTU_PRESCRICAO_MEDICA - "ID_PRESCRICAO_MEDICA","ID_UNID_HOSPITAL","ID_CONSULTA","ID_MEDICAMENTO","DS_POSOLOGIA","DS_VIA","DS_OBSERVACAO_USO","QT_MEDICAMENTO","NM_USUARIO","DT_CADASTRO"

define_and_impl_sql_insertable!(
    T_RHSTU_MEDICAMENTO {
        ID_MEDICAMENTO: u64,
        NM_MEDICAMENTO: String,
        DS_DETALHADA_MEDICAMENTO: String,
        NR_CODIGO_BARRAS: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_PRESCRICAO_MEDICA {
        ID_PRESCRICAO_MEDICA: u64,
        ID_UNID_HOSPITAL: u64,
        ID_CONSULTA: u64,
        ID_MEDICAMENTO: u64,
        DS_POSOLOGIA: String,
        DS_VIA: String,
        DS_OBSERVACAO_USO: String,
        QT_MEDICAMENTO: u64,
        NM_USUARIO: String,
        DT_CADASTRO: String
    }
);

#[derive(Deserialize, Debug, Serialize)]
struct MedicineCategories {
    id: u64,
    descricao: String,
    ativo: String,
}


pub(crate) async fn get_medicines(
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_MEDICAMENTO> {

    let client = Arc::new(BularioClient::new());
    let mut categories = client.fetch_categories().await.expect("Error fetching categories");
    categories.sort_by(|a, b| a.id.cmp(&b.id));
    // Shared data structure for all tasks to aggregate their results
    let aggregated_results = Arc::new(Mutex::new(Vec::new()));

    // Spawn a new task for each category
    let tasks: Vec<_> = categories.clone()
        .into_iter()
        .map(|category| {
            let client = client.clone();
        


            let shared_results = aggregated_results.clone();
            let pb = Arc::new(m.add(ProgressBar::new(1))); // Assuming each category has 1 unit of work
            pb.set_prefix(format!("Medicines {}/{}:", category.id,categories.len()+1));
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("{prefix} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg:.red}").expect("Error setting progress bar style")
                    .progress_chars("#>-"),
            );
    

            let task = task::spawn(spawn_theard( client.clone(),category.id as usize, pb.clone(), main_pb.clone(), shared_results));

            
            task
        })
        .collect();

    // Wait for all tasks to complete
    futures::future::join_all(tasks).await;

    let locked_results = aggregated_results.lock().unwrap();
    
    let pb_helper = ProgressBarHelper::new(m, locked_results.len() * 2, "Medicines:".to_string());
    let pb = &pb_helper.pb;
    

    let mut medicines: Vec<T_RHSTU_MEDICAMENTO> = Vec::new();

    for medicine_data in locked_results.iter() {
        medicines.push(medicine_data.clone());
        pb.inc(1);
    }

    let generator = SqlGenerator::new(medicines.clone());
    let _ = generator.write_to_file(pb);

    pb_helper.finish();

    medicines
}

async fn spawn_theard(    
    client: Arc<BularioClient>,
    category_id: usize,
    pb: Arc<ProgressBar>,
    main_pb: Arc<ProgressBar>,
    shared_results: Arc<Mutex<Vec<T_RHSTU_MEDICAMENTO>>>,
){
        let main = main_pb.clone();
        let results = process_category(&client, category_id as usize, &pb, main).await;
        {
            let mut shared = shared_results.lock().unwrap();
            shared.extend(results);
        }
    }

async fn process_category(
    client: &BularioClient,
    category_id: usize,
    pb: &ProgressBar,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_MEDICAMENTO> {
    let body_first = match client.fetch_medicines_by_category(category_id, 1).await {
        Ok(result) => result,
        Err(_) => {
            // pb.println(format!("Error: {}", error));
            pb.inc(1);
            return Vec::new();
        }
        
    };

    if body_first.totalElements == 0 {
        // pb.set_style(ProgressStyle::default_bar()
        //     .template("{msg}").expect("Error setting progress bar style")
        // );
        // pb.finish_with_message(format!("Medicine {}/12 is empty", category_id));
        return Vec::new();
    }

    let mut medicines = Vec::new();
    

    // Set the progress bar's total to the total number of pages
    pb.set_length(body_first.totalElements as u64);

    for page in 1..body_first.totalPages {
        pb.set_message("");
        let result = match client.fetch_medicines_by_category(category_id, page as u64).await {
            Ok(result) => result,
            Err(_) => {
                pb.set_message(format!("Error fetching page {}, skiping", page));
                pb.inc((body_first.totalElements/body_first.totalPages) as u64);
                continue;
            }
        };

        for medicine in result.content {
            let medicine_data = T_RHSTU_MEDICAMENTO {
                ID_MEDICAMENTO: medicine.idProduto,
                NM_MEDICAMENTO: medicine.nomeProduto.substring(0, 49).to_string(),
                DS_DETALHADA_MEDICAMENTO: medicine.expediente,
                NR_CODIGO_BARRAS: medicine.numeroRegistro,
                DT_CADASTRO: current_timestamp(),
                NM_USUARIO: "1".to_string(),
            };
            medicines.push(medicine_data);
            pb.inc(1);
            main_pb.inc(1);
        }

    }


    medicines
}

// MedicalPrescription
// - T_RHSTU_PRESCRICAO_MEDICA - "ID_PRESCRICAO_MEDICA","ID_UNID_HOSPITAL","ID_CONSULTA","ID_MEDICAMENTO","DS_POSOLOGIA","DS_VIA","DS_OBSERVACAO_USO","QT_MEDICAMENTO","NM_USUARIO","DT_CADASTRO"
pub(crate) async fn generate_medical_prescription(total: usize, medicines: Vec<T_RHSTU_MEDICAMENTO>,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) {
    let pb_helper = ProgressBarHelper::new(m, total * 2, "Medical Prescription:".to_string());
    let pb = &pb_helper.pb;
    let mut prescripitions: Vec<T_RHSTU_PRESCRICAO_MEDICA> = Vec::new();

    for i in 0..total {
        let medicine = medicines.choose(&mut rand::thread_rng()).unwrap();

        let medical_prescription = T_RHSTU_PRESCRICAO_MEDICA {
            ID_PRESCRICAO_MEDICA: i as u64,
            ID_UNID_HOSPITAL: rand::thread_rng().gen_range(1..100) as u64,
            ID_CONSULTA: rand::thread_rng().gen_range(1..100) as u64,
            ID_MEDICAMENTO: medicine.ID_MEDICAMENTO,
            DS_POSOLOGIA: "DS_POSOLOGIA".to_string(),
            DS_VIA: "DS_VIA".to_string(),
            DS_OBSERVACAO_USO: "DS_OBSERVACAO_USO".to_string(),
            QT_MEDICAMENTO: rand::thread_rng().gen_range(1..100) as u64,
            NM_USUARIO: "NM_USUARIO".to_string(),
            DT_CADASTRO: current_timestamp(),
        };

        prescripitions.push(medical_prescription);

        pb.inc(1);
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(prescripitions.clone());
    let _ = generator.write_to_file(pb);

    
    pb_helper.finish();
}

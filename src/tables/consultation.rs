use std::sync::Arc;

use fake::{faker::name::en::Name, Fake};
use indicatif::{MultiProgress, ProgressBar};
use rand::{seq::SliceRandom, Rng};

use crate::{
    common::ProgressBarHelper, define_and_impl_sql_insertable, sql_generator::SqlGenerator,
};
// - T_RHSTU_CONSULTA - "ID_UNID_HOSPITAL","ID_CONSULTA","ID_PACIENTE","ID_FUNC","DT_HR_CONSULTA","NR_CONSULTORIO","DT_CADASTRO","NM_USUARIO"

define_and_impl_sql_insertable!(
    T_RHSTU_CONSULTA {
        ID_UNID_HOSPITAL: u32,
        ID_CONSULTA: u32,
        ID_PACIENTE: u32,
        ID_FUNC: u32,
        DT_HR_CONSULTA: String,
        NR_CONSULTORIO: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_FORMA_PAGAMENTO {
        ID_FORMA_PAGTO: u32,
        NM_FORMA_PAGTO: String,
        DS_FORMA_PAGTO: String,
        ST_FORMA_PAGTO: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    },
    T_RHSTU_CONSULTA_FORMA_PAGTO {
        ID_CONSULTA_FORMA_PAGTO: u32,
        ID_UNID_HOSPITAL: u32,
        ID_CONSULTA: u32,
        ID_PACIENTE_PS: u32,
        ID_FORMA_PAGTO: u32,
        DT_PAGTO_CONSULTA: String,
        ST_PAGTO_CONSULTA: String,
        DT_CADASTRO: String,
        NM_USUARIO: String
    }
);

pub(crate) async fn generate_consultations(
    total: usize,
    total_hospitals: usize,
    total_patients: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_CONSULTA> {
    // println!("Generating consultations...");
    let mut consultations: Vec<T_RHSTU_CONSULTA> = Vec::new();
    let mut rng = rand::thread_rng();

    let pb_helper = ProgressBarHelper::new(m, total, "Consultations:".to_string());
    let pb = &pb_helper.pb;

    for i in 0..total {
        let consultation = T_RHSTU_CONSULTA {
            ID_UNID_HOSPITAL: rng.gen_range(1..total_hospitals) as u32,
            ID_CONSULTA: i as u32,
            ID_PACIENTE: rng.gen_range(1..total_patients) as u32,
            ID_FUNC: rng.gen_range(1..100) as u32,
            DT_HR_CONSULTA: chrono::Local::now().to_string(),
            NR_CONSULTORIO: rng.gen_range(1..100).to_string(),
            DT_CADASTRO: chrono::Local::now().to_string(),
            NM_USUARIO: "1".to_string(),
        };

        consultations.push(consultation);
        pb.inc(1); // Increment the progress bar
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(consultations.clone());
    let _ = generator.write_to_file();

    pb_helper.finish();
    consultations
}

pub(crate) async fn generate_payment_methods(
    total: usize,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) -> Vec<T_RHSTU_FORMA_PAGAMENTO> {
    // println!("Generating payment methods...");
    let mut payment_methods: Vec<T_RHSTU_FORMA_PAGAMENTO> = Vec::new();

    let pb_helper = ProgressBarHelper::new(m, total, "Payment Methods:".to_string());
    let pb = &pb_helper.pb;

    for i in 0..total {
        let payment_method = T_RHSTU_FORMA_PAGAMENTO {
            ID_FORMA_PAGTO: i as u32,
            NM_FORMA_PAGTO: Name().fake(),
            DS_FORMA_PAGTO: Name().fake(),
            ST_FORMA_PAGTO: ["Ativo", "Inativo"]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_string(),
            DT_CADASTRO: chrono::Local::now().to_string(),
            NM_USUARIO: "1".to_string(),
        };

        payment_methods.push(payment_method);
        pb.inc(1); // Increment the progress bar
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(payment_methods.clone());
    let _ = generator.write_to_file();

    pb_helper.finish();
    payment_methods
}

pub(crate) async fn generate_consultation_payment_methods(
    total: usize,
    payment_methods: Vec<T_RHSTU_FORMA_PAGAMENTO>,
    consultations: Vec<T_RHSTU_CONSULTA>,
    m: Arc<MultiProgress>,
    main_pb: Arc<ProgressBar>,
) {
    // println!("Generating consultation payment methods...");
    let mut rng = rand::thread_rng();

    let pb_helper = ProgressBarHelper::new(m, total, "Consultation Payment Methods:".to_string());
    let pb = &pb_helper.pb;
    let mut consultation_payment_methods: Vec<T_RHSTU_CONSULTA_FORMA_PAGTO> = Vec::new();

    for i in 0..total {
        let payment_method = payment_methods.choose(&mut rng).unwrap();

        let consultation = consultations.choose(&mut rng).unwrap();

        let consultation_payment_method = T_RHSTU_CONSULTA_FORMA_PAGTO {
            ID_CONSULTA_FORMA_PAGTO: i as u32,
            ID_UNID_HOSPITAL: consultation.ID_UNID_HOSPITAL,
            ID_CONSULTA: consultation.ID_CONSULTA,
            ID_PACIENTE_PS: consultation.ID_PACIENTE,
            ID_FORMA_PAGTO: payment_method.ID_FORMA_PAGTO,
            DT_PAGTO_CONSULTA: chrono::Local::now().to_string(),
            ST_PAGTO_CONSULTA: ["Ativo", "Inativo"].choose(&mut rng).unwrap().to_string(),
            DT_CADASTRO: chrono::Local::now().to_string(),
            NM_USUARIO: "1".to_string(),
        };

        consultation_payment_methods.push(consultation_payment_method);
        pb.inc(1); // Increment the progress bar
        main_pb.inc(1);
    }

    let generator = SqlGenerator::new(consultation_payment_methods.clone());
    let _ = generator.write_to_file();

    pb_helper.finish();
}

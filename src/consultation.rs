use fake::{faker::name::en::Name, Fake};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
// - T_RHSTU_CONSULTA - "ID_UNID_HOSPITAL","ID_CONSULTA","ID_PACIENTE","ID_FUNC","DT_HR_CONSULTA","NR_CONSULTORIO","DT_CADASTRO","NM_USUARIO"
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(crate) struct Consultation {
    pub(crate) ID_UNID_HOSPITAL: u32,
    pub(crate) ID_CONSULTA: u32,
    pub(crate) ID_PACIENTE: u32,
    pub(crate) ID_FUNC: u32,
    pub(crate) DT_HR_CONSULTA: String,
    pub(crate) NR_CONSULTORIO: String,
    pub(crate) DT_CADASTRO: String,
    pub(crate) NM_USUARIO: String,
}

// - T_RHSTU_FORMA_PAGAMENTO - "ID_FORMA_PAGTO","NM_FORMA_PAGTO","DS_FORMA_PAGTO","ST_FORMA_PAGTO","DT_CADASTRO","NM_USUARIO"
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(crate) struct PaymentMethod {
    ID_FORMA_PAGTO: u32,
    NM_FORMA_PAGTO: String,
    DS_FORMA_PAGTO: String,
    ST_FORMA_PAGTO: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

// - T_RHSTU_CONSULTA_FORMA_PAGTO - "ID_CONSULTA_FORMA_PAGTO","ID_UNID_HOSPITAL","ID_CONSULTA","ID_PACIENTE_PS","ID_FORMA_PAGTO","DT_PAGTO_CONSULTA","ST_PAGTO_CONSULTA","DT_CADASTRO","NM_USUARIO"
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub(crate) struct ConsultationPaymentMethod {
    ID_CONSULTA_FORMA_PAGTO: u32,
    ID_UNID_HOSPITAL: u32,
    ID_CONSULTA: u32,
    ID_PACIENTE_PS: u32,
    ID_FORMA_PAGTO: u32,
    DT_PAGTO_CONSULTA: String,
    ST_PAGTO_CONSULTA: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

pub(crate) async fn generate_consultations(
    total: usize,
    total_hospitals: usize,
    total_patients: usize,
) -> Vec<Consultation> {
    let mut writer = csv::Writer::from_path("data/consultation.csv").unwrap();
    let mut consultations: Vec<Consultation> = Vec::new();
    let mut rng = rand::thread_rng();

    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Generating consultations...");

    for i in 0..total {
        let consultation = Consultation {
            ID_UNID_HOSPITAL: rng.gen_range(1..total_hospitals) as u32,
            ID_CONSULTA: i as u32,
            ID_PACIENTE: rng.gen_range(1..total_patients) as u32,
            ID_FUNC: rng.gen_range(1..100) as u32,
            DT_HR_CONSULTA: chrono::Local::now().to_string(),
            NR_CONSULTORIO: rng.gen_range(1..100).to_string(),
            DT_CADASTRO: chrono::Local::now().to_string(),
            NM_USUARIO: "1".to_string(),
        };

        writer.serialize(&consultation).unwrap();
        consultations.push(consultation);
        pb.inc(1); // Increment the progress bar
    }
    pb.finish_with_message("Consultations generated!");
    consultations
}

pub(crate) async fn generate_payment_methods(total: usize) -> Vec<PaymentMethod> {
    let mut writer = csv::Writer::from_path("data/payment_method.csv").unwrap();
    let mut payment_methods: Vec<PaymentMethod> = Vec::new();

    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Generating payment methods...");
    for i in 0..total {
        let payment_method = PaymentMethod {
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

        writer.serialize(&payment_method).unwrap();
        payment_methods.push(payment_method);
        pb.inc(1); // Increment the progress bar
    }
    pb.finish_with_message("Payment methods generated!");
    payment_methods
}

pub(crate) async fn generate_consultation_payment_methods(
    total: usize,
    payment_methods: Vec<PaymentMethod>,
    consultations: Vec<Consultation>,
) {
    let mut rng = rand::thread_rng();

    let mut writer = csv::Writer::from_path("data/consultation_payment_method.csv").unwrap();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.set_message("Generating consultation payment methods...");
    for i in 0..total {
        let payment_method = payment_methods.choose(&mut rng).unwrap();

        let consultation = consultations.choose(&mut rng).unwrap();

        let consultation_payment_method = ConsultationPaymentMethod {
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

        writer.serialize(&consultation_payment_method).unwrap();
        pb.inc(1); // Increment the progress bar
    }
    pb.finish_with_message("Consultation payment methods generated!");
}

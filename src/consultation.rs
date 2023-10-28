use fake::{faker::name::en::Name, Fake};
use rand::{seq::SliceRandom, Rng};
// - T_RHSTU_CONSULTA - "ID_UNID_HOSPITAL","ID_CONSULTA","ID_PACIENTE","ID_FUNC","DT_HR_CONSULTA","NR_CONSULTORIO","DT_CADASTRO","NM_USUARIO"
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
pub(crate) struct PaymentMethod {
    ID_FORMA_PAGTO: u32,
    NM_FORMA_PAGTO: String,
    DS_FORMA_PAGTO: String,
    ST_FORMA_PAGTO: String,
    DT_CADASTRO: String,
    NM_USUARIO: String,
}

// - T_RHSTU_CONSULTA_FORMA_PAGTO - "ID_CONSULTA_FORMA_PAGTO","ID_UNID_HOSPITAL","ID_CONSULTA","ID_PACIENTE_PS","ID_FORMA_PAGTO","DT_PAGTO_CONSULTA","ST_PAGTO_CONSULTA","DT_CADASTRO","NM_USUARIO"
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

pub(crate) fn generate_consultations(total: usize) -> Vec<Consultation> {
    let mut consultations: Vec<Consultation> = Vec::with_capacity(total);
    let mut rng = rand::thread_rng();

    for _ in 0..total {
        let consultation_id = rng.gen_range(1..100) as u32;
        let consultation_patient_id = rng.gen_range(1..100) as u32;
        let consultation_doctor_id = rng.gen_range(1..100) as u32;
        let consultation_hospital_id = rng.gen_range(1..100) as u32;
        let consultation_date = chrono::Local::now().to_string();
        let consultation_room = rng.gen_range(1..100) as u32;
        let consultation_register_date = chrono::Local::now().to_string();

        consultations.push(Consultation {
            ID_UNID_HOSPITAL: consultation_hospital_id,
            ID_CONSULTA: consultation_id,
            ID_PACIENTE: consultation_patient_id,
            ID_FUNC: consultation_doctor_id,
            DT_HR_CONSULTA: consultation_date,
            NR_CONSULTORIO: consultation_room.to_string(),
            DT_CADASTRO: consultation_register_date,
            NM_USUARIO: "1".to_string(),
        });
    }

    consultations
}

pub(crate) fn generate_payment_methods(total: usize) -> Vec<PaymentMethod> {
    let mut payment_methods: Vec<PaymentMethod> = Vec::with_capacity(total);
    let mut rng = rand::thread_rng();

    for _ in 0..total {
        let payment_method_id = rng.gen_range(1..100) as u32;
        let payment_method_name = Name().fake();
        let payment_method_description = Name().fake();
        let payment_method_status = ["Ativo", "Inativo"]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        let payment_method_register_date = chrono::Local::now().to_string();

        payment_methods.push(PaymentMethod {
            ID_FORMA_PAGTO: payment_method_id,
            NM_FORMA_PAGTO: payment_method_name,
            DS_FORMA_PAGTO: payment_method_description,
            ST_FORMA_PAGTO: payment_method_status,
            DT_CADASTRO: payment_method_register_date,
            NM_USUARIO: "1".to_string(),
        });
    }

    payment_methods
}

pub(crate) fn generate_consultation_payment_methods(
    total: usize,
    payment_methods: Vec<PaymentMethod>,
    consultations: Vec<Consultation>,
) -> Vec<ConsultationPaymentMethod> {
    let mut consultation_payment_methods: Vec<ConsultationPaymentMethod> =
        Vec::with_capacity(total);
    let mut rng = rand::thread_rng();

    for _ in 0..total {
        let consultation_payment_method_id = rng.gen_range(1..100) as u32;
        let consultation_payment_method_consultation_id = consultations
            .choose(&mut rand::thread_rng())
            .unwrap()
            .ID_CONSULTA;
        let consultation_payment_method_patient_id = rng.gen_range(1..100) as u32;
        let consultation_payment_method_payment_method_id = payment_methods
            .choose(&mut rand::thread_rng())
            .unwrap()
            .ID_FORMA_PAGTO;
        let consultation_payment_method_date = chrono::Local::now().to_string();
        let consultation_payment_method_status = ["Ativo", "Inativo"]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();
        let consultation_payment_method_register_date = chrono::Local::now().to_string();

        consultation_payment_methods.push(ConsultationPaymentMethod {
            ID_CONSULTA_FORMA_PAGTO: consultation_payment_method_id,
            ID_UNID_HOSPITAL: 1,
            ID_CONSULTA: consultation_payment_method_consultation_id,
            ID_PACIENTE_PS: consultation_payment_method_patient_id,
            ID_FORMA_PAGTO: consultation_payment_method_payment_method_id,
            DT_PAGTO_CONSULTA: consultation_payment_method_date,
            ST_PAGTO_CONSULTA: consultation_payment_method_status,
            DT_CADASTRO: consultation_payment_method_register_date,
            NM_USUARIO: "1".to_string(),
        });
    }

    consultation_payment_methods
}

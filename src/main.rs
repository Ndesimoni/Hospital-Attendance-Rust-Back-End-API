use axum::{
    Router,
    routing::{delete, get, post, put},
};

use std::sync::{Arc, Mutex};

use task_flow_api::{
    db::create_pool,
    handlers::{create_patients, get_all_patients, get_patients_by_id, update_patients_detail},
    repository::{
        self, patient_repository::PatientRepository,
        postgres_patient_repository::PostgresPatientRepository,
    },
    services::patient_service::PatientService,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = create_pool().await;

    let repository = Arc::new(PostgresPatientRepository::new(pool));

    let repository: Arc<dyn PatientRepository> = repository;

    // 3. Create service and inject repository
    let patient_service = Arc::new(PatientService::new(repository));

    let app = Router::new()
        .route("/patients", get(get_all_patients))
        .route("/patients/{id}", get(get_patients_by_id))
        .route("/patients", post(create_patients))
        .route("/patients/{id}", put(update_patients_detail))
        // .route("/patients/{id}", delete(delete_patient))
        // .route("/patients/{id}/visits", post(create_visit))
        // .route("/visits", get(visits))
        // .route("/patient/{id}/visit", get(patients_visit))
        .with_state(patient_service);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("Server started on http://127.0.0.1:4000");

    axum::serve(listener, app).await.unwrap();
}

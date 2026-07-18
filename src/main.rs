use axum::{
    Router,
    routing::{delete, get, post, put},
};

use std::sync::Arc;

use task_flow_api::{
    db::create_pool,
    handlers::{
        create_patients, create_visit, get_all_patients, get_patients_by_id, update_patients_detail,
    },
    repository::{
        patient_repository::PatientRepository,
        postgres_patient_repository::PostgresPatientRepository,
        postgres_visit_repository::PostgresVisitRepository,
        visit_repository::{self, VisitRepository},
    },
    services::{patient_service::PatientService, visit_service::VisitService},
    state::AppState,
};

////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = create_pool().await;

    //*for working with patients data
    let repository = Arc::new(PostgresPatientRepository::new(pool.clone()));

    let repository: Arc<dyn PatientRepository> = repository;

    // Create service and inject repository
    let patient_service = Arc::new(PatientService::new(repository));

    //*for working with visit data
    let visit_repository = Arc::new(PostgresVisitRepository::new(pool.clone()));

    let visit_repository: Arc<dyn VisitRepository> = visit_repository;

    //Create visit and inject repository
    let visit_service = Arc::new(VisitService::new(visit_repository));

    let app_state = AppState {
        patient_service,
        visit_service,
    };

    let app = Router::new()
        .route("/patients", get(get_all_patients))
        .route("/patients/{id}", get(get_patients_by_id))
        .route("/patients", post(create_patients))
        .route("/patients/{id}", put(update_patients_detail))
        .route("/patients/{id}/visits", post(create_visit))
        // .route("/patients/{id}", delete(delete_patient))
        // .route("/visits", get(visits))
        // .route("/patient/{id}/visit", get(patients_visit))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("Server started on http://127.0.0.1:4000");

    axum::serve(listener, app).await.unwrap();
}

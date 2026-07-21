use axum::{
    Router,
    routing::{delete, get, post, post_service, put},
};

use std::sync::Arc;

use task_flow_api::{
    db::create_pool,
    handlers::{
        auth_handler, create_patients, create_visit, get_all_patients, get_all_visits,
        get_patient_visit, get_patients_by_id, update_patients_detail, update_visit,
    },
    repositories::{
        auth_repository::AuthRepository,
        patient_repository::PatientRepository,
        postgres_auth_repository::PostgresAuthRepository,
        postgres_patient_repository::PostgresPatientRepository,
        postgres_visit_repository::PostgresVisitRepository,
        visit_repository::{self, VisitRepository},
    },
    services::{
        auth_service::AuthService, patient_service::PatientService, visit_service::VisitService,
    },
    state::AppState,
};

////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = create_pool().await;

    //*for working with patients data
    let patient_repository = Arc::new(PostgresPatientRepository::new(pool.clone()));

    let patient_repository: Arc<dyn PatientRepository> = patient_repository;

    // Create patient and inject repository
    let patient_service = Arc::new(PatientService::new(patient_repository.clone()));

    ///////////////////////////////////////
    //* for working with visit data
    let visit_repository = Arc::new(PostgresVisitRepository::new(pool.clone()));

    let visit_repository: Arc<dyn VisitRepository> = visit_repository;

    // Create visit and inject repository
    let visit_service = Arc::new(VisitService::new(
        visit_repository,
        patient_repository.clone(),
    ));

    //////////////////////////
    let auth_repository = Arc::new(PostgresAuthRepository::new(pool.clone()));

    let auth_repository: Arc<dyn AuthRepository> = auth_repository;

    let auth_service = Arc::new(AuthService::new(auth_repository.clone()));

    //* app state
    let app_state = AppState {
        patient_service,
        visit_service,
        auth_service,
    };

    let app = Router::new()
        .route("/patients", get(get_all_patients))
        .route("/patients/{id}", get(get_patients_by_id))
        .route("/patients", post(create_patients))
        .route("/patients/{id}", put(update_patients_detail))
        .route("/patients/{id}/visits", post(create_visit))
        .route("/visits", get(get_all_visits))
        .route("/patients/{id}/visits", get(get_patient_visit))
        .route("/visits/{id}", put(update_visit))
        .route("/create_user", post(auth_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("Server started on http://127.0.0.1:4000");

    axum::serve(listener, app).await.unwrap();
}

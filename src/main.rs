use axum::{
    Router,
    routing::{delete, get, post, put},
};

use std::sync::{Arc, Mutex};

use task_flow_api::{
    db::create_pool,
    handlers::{create_patient, get_all_patients},
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = create_pool().await;

    let app = Router::new()
        .route("/patients", post(create_patient))
        .route("/patients", get(get_all_patients))
        // .route("/patients/{id}", get(find_patient))
        // .route("/patients/{id}", put(update_patient_details))
        // .route("/patients/{id}", delete(delete_patient))
        // .route("/patients/{id}/visits", post(create_visit))
        // .route("/visits", get(visits))
        // .route("/patient/{id}/visit", get(patients_visit))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    println!("Server started on http://127.0.0.1:4000");

    axum::serve(listener, app).await.unwrap();
}

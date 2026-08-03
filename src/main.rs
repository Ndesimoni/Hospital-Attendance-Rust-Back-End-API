use axum::{
    Router, middleware,
    routing::{get, post, put},
};

use std::sync::Arc;

use task_flow_api::{
    db::create_pool,
    handlers::{
        auth_handler, create_patients, create_user, create_visit, get_all_patients, get_all_user,
        get_all_visits, get_patient_visit, get_patients_by_id, health_check, login,
        login_after_otp_verification, patient_login, update_patients_detail, update_user,
        update_visit,
    },
    middleware::{
        auth::auth_middleware,
        require_doctor,
        role::{require_admin, require_receptionist},
    },
    repositories::{
        auth_repository::AuthRepository, patient_otp_repository::PatientOtpRepository,
        patient_repository::PatientRepository, postgres_auth_repository::PostgresAuthRepository,
        postgres_patient_otp_repository::PostgresPatientOtpRepository,
        postgres_patient_repository::PostgresPatientRepository,
        postgres_user_role_repository::PostgresUserRoleRepository,
        postgres_visit_repository::PostgresVisitRepository,
        user_role_repository::UserRoleRepository, visit_repository::VisitRepository,
    },
    services::{
        auth_service::AuthService, patient_opt_service::PatientOtpService,
        patient_service::PatientService, user_role_service::UserRoleServices,
        visit_service::VisitService,
    },
    state::AppState,
};

////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = create_pool().await;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    //*for working with patients data
    let patient_repository = Arc::new(PostgresPatientRepository::new(pool.clone()));

    let patient_repository: Arc<dyn PatientRepository> = patient_repository;

    let patient_service = Arc::new(PatientService::new(patient_repository.clone()));

    ///////////////////////////////////////////////////////////
    //* for working with visit data
    let visit_repository = Arc::new(PostgresVisitRepository::new(pool.clone()));

    let visit_repository: Arc<dyn VisitRepository> = visit_repository;

    let visit_service = Arc::new(VisitService::new(
        visit_repository,
        patient_repository.clone(),
    ));

    //////////////////////////
    //* for working with a create_user data
    let auth_repository = Arc::new(PostgresAuthRepository::new(pool.clone()));

    let auth_repository: Arc<dyn AuthRepository> = auth_repository;

    let auth_service = Arc::new(AuthService::new(auth_repository.clone()));

    ////////////////////////////////
    //* for working with user role */
    let user_role_repository = Arc::new(PostgresUserRoleRepository::new(pool.clone()));

    let role_repository: Arc<dyn UserRoleRepository> = user_role_repository;

    let role_service = Arc::new(UserRoleServices::new(
        auth_repository.clone(),
        role_repository,
    ));

    ////////////////////////////////////////////////////
    //* for working with patient login */
    let patient_opt_repository = Arc::new(PostgresPatientOtpRepository::new(pool.clone()));

    let otp_service: Arc<dyn PatientOtpRepository> = patient_opt_repository;

    let otp_service = Arc::new(PatientOtpService::new(
        otp_service,
        patient_repository,
        jwt_secret.clone(),
    ));

    ////////////////////////////////////////////////////

    //* app state
    let app_state = AppState {
        patient_service,
        visit_service,
        auth_service,
        jwt_secret,
        role_service,
        otp_service,
    };

    //*health check route  */
    let health_check = Router::new().route("/health", get(health_check));

    //*public routes */
    //user public routes
    let public_routes = Router::new()
        .route("/create_user", post(auth_handler))
        .route("/login", post(login));

    //patients public routes
    let patient_public_routes = Router::new().route("/login", post(patient_login));
    let patient_log_after_otp_verification =
        Router::new().route("/verify-otp", post(login_after_otp_verification));

    //* routes for all authenticated users */
    let authenticated_routes = Router::new()
        .route("/patients", get(get_all_patients))
        .route("/patients/{id}", get(get_patients_by_id))
        .route("/visits", get(get_all_visits))
        .route("/patients/{id}/visits", get(get_patient_visit))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    //* receptionist only routes */
    let receptionist_routes = Router::new()
        .route("/patients", post(create_patients))
        .route("/patients", get(get_all_patients))
        .route("/visits", get(get_all_visits))
        .route("/patients/{id}", put(update_patients_detail))
        .route("/patients/{id}/visits", post(create_visit))
        .layer(middleware::from_fn(require_receptionist))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    //* doctor only routes */
    let doctor_routes = Router::new()
        .route("/visits/{id}", put(update_visit))
        .layer(middleware::from_fn(require_doctor))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    //* admin only routes */
    let admin_route = Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", put(update_user))
        .route("/users", get(get_all_user))
        .route("/patients", get(get_all_patients))
        .route_layer(middleware::from_fn(require_admin))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    let app = Router::new()
        .merge(public_routes)
        .merge(health_check)
        .nest("/patients", patient_public_routes)
        .nest("/patients", patient_log_after_otp_verification)
        .nest("/authenticate", authenticated_routes)
        .nest("/admin", admin_route)
        .nest("/reception", receptionist_routes)
        .nest("/doctor", doctor_routes)
        .with_state(app_state);

    let host = std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("APP_PORT").unwrap_or_else(|_| "4000".to_string());

    let address = std::format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    tracing::info!("Server started on http://{}", address);

    axum::serve(listener, app).await.unwrap();
}

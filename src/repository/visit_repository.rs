use crate::models::{CreateVisit, NewVisit, Visit};

pub trait VisitRepository: Send + Sync {
    async fn create_visit_trait(&self, payload: NewVisit) -> Result<Visit, sqlx::Error>;

    // async fn find_all_visits_trait();

    // async fn find_patient_visit_trait();

    // async fn update_patient_visit_trait();
}

use crate::models::{CreateVisit, NewVisit, UpdateVisit, Visit};

#[async_trait::async_trait]
pub trait VisitRepository: Send + Sync {
    async fn create_visit_trait(&self, payload: NewVisit) -> Result<Visit, sqlx::Error>;

    async fn get_all_visits_trait(&self) -> Result<Vec<Visit>, sqlx::Error>;

    async fn get_a_patient_visits_trait(&self, id: i32) -> Result<Vec<Visit>, sqlx::Error>;

    //todo implement this trait so i check if patient exist before 
    // async fn get_visit_by_id(&self, id: i32) -> Result<Option<Visit>, sqlx::Error>;


    async fn update_a_visit_trait(
        &self,
        id: i32,
        payload: UpdateVisit,
    ) -> Result<Option<Visit>, sqlx::Error>;
}

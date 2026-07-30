use serde::Deserialize;

use crate::models::AppError;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

//setting the default page and limit for pagination
impl Pagination {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(10)
    }

    pub fn validation(&self) -> Result<(), AppError> {
        if self.page() == 0 {
            return Err(AppError::BadRequest(String::from(
                "page must be greater than 0",
            )));
        }

        if self.limit() == 0 {
            return Err(AppError::BadRequest(String::from(
                "limit  must be greater than 0",
            )));
        }

        if self.limit() > 100 {
            return Err(AppError::BadRequest(String::from(
                "limit must be less than 100",
            )));
        }

        Ok(())
    }
}

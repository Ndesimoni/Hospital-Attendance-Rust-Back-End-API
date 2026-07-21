mod auth;
mod patient;
mod users;
mod visit;
pub use auth::{CreateUser, RegisterRequest};
pub use patient::{CreatePatient, Patient, UpdatePatient};
pub use users::Users;
pub use visit::{CreateVisit, NewVisit, UpdateVisit, Visit};

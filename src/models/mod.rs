mod auth;
mod patient;
mod users;
mod visit;
pub use auth::{LoginRequest, LoginResponse, RegisterRequest};
pub use patient::{CreatePatient, Patient, UpdatePatient};
pub use users::{CreateUser, Users};
pub use visit::{CreateVisit, NewVisit, UpdateVisit, Visit};

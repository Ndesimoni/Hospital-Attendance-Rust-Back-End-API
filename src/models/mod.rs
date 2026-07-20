mod patient;
mod users;
mod visit;
pub use patient::{CreatePatient, Patient, UpdatePatient};
pub use users::{CreateUsers, Users};
pub use visit::{CreateVisit, NewVisit, UpdateVisit, Visit};

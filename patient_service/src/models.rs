// Structs: Serialize/Deserialize/Validate
use serde::{Deserialize, Serialize};

// TODO: modify this to a proper MongoDB schema (current is only for testing)
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreatePatientDto {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) date_of_birth: String, // Or use a Date type from the 'chrono' crate
    pub(crate) email: String,
    pub(crate) phone: String,
    pub(crate) address: Option<String>, // Option makes it optional in the JSON
}

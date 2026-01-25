// Structs: Serialize/Deserialize/Validate

use serde::{Deserialize, Serialize};

// TODO: modify this to a proper MongoDB schema (current is only for testing)
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateClinicDto {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) phone: String,
    pub(crate) address: Option<String>, // Option makes it optional in the JSON
}

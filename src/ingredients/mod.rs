mod measure;

use measure::Measure;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    pub ingredient: String,
    pub measure: Measure,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
}

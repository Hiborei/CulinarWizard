use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Measure {
    value: f32,
    scale: Scale,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Scale {
    Quantity,
    Grams,
    Cup,
    Can,
    Mililiters,
    Teaspoon,
    Tablespoon,
    ABit,
    Slice,
}

use serde::Deserialize;
use serde_json;
use std::fs;

#[derive(Deserialize)]
struct Meal {
    id_meal: String, // I think making a custom deserialize is better here than keeping the fields as "strIngredient1" etc. -.-
}

fn main() {
    println!("Starting CulinarWizard!");
    let data = fs::read_to_string("resources/meals.json").expect("File doesn't exist!");
    // Parse it eventually
    //let meals = serde_json::from_str(&data).expect("Couldn't parse json!");
    println!("{}", data);
}

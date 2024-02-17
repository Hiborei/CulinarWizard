use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Ingredient {
    ingredient: String,
    measure: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Meal {
    meal_id: u16,
    meal_name: String,
    category: String,
    area: String,
    instructions: Vec<String>,
    meal_thumb: String,
    tags: Vec<Tag>,
    youtube: String,
    source: String,
    ingredient_list: Vec<Ingredient>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Meals {
    meals: Vec<Meal>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Tag {
    Beans,
    Onthego,
    Soup,
    Warming,
    Alcoholic,
    BBQ,
    Baking,
    Breakfast,
    Brunch,
    Bun,
    Cake,
    Calorific,
    Caramel,
    Casserole,
    Cheasy,
    Chilli,
    Chocolate,
    Christmas,
    Curry,
    Dairy,
    DateNight,
    Desert,
    DinnerParty,
    Easter,
    Egg,
    Expensive,
    Fish,
    Fresh,
    Fruity,
    Fusion,
    Glazed,
    Greasy,
    Halloween,
    HangoverFood,
    Heavy,
    HighFat,
    Keto,
    Light,
    LowCalorie,
    LowCarbs,
    MainMeal,
    Meat,
    Mild,
    Nutty,
    Paella,
    Paleo,
    Pasta,
    Pie,
    Pudding,
    Pulse,
    Salad,
    Sandwich,
    Sausages,
    Savory,
    Seafood,
    Shellfish,
    SideDish,
    Snack,
    Sour,
    Speciality,
    Spicy,
    Stew,
    Streetfood,
    StrongFlavor,
    Summer,
    Sweet,
    Tart,
    Treat,
    UnHealthy,
    Vegan,
    Vegetables,
    Vegetarian,
    Warm,
    Pancake,
    Rice,
}

fn main() {
    println!("Starting CulinarWizard!");
    let data = fs::read_to_string("resources/meals.json").expect("File doesn't exist!");
    let meals = serde_json::from_str::<Meals>(&data).expect("Couldn't parse json!");
    println!("Success! {:?}", meals.meals[0]);
}

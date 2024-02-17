use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case, dead_code)]
pub struct OldMeal {
    idMeal: String,
    strMeal: String,
    strCategory: String,
    strArea: String,
    strInstructions: String,
    strMealThumb: String,
    strTags: String,
    strYoutube: String,
    strIngredient1: String,
    strIngredient2: String,
    strIngredient3: String,
    strIngredient4: String,
    strIngredient5: String,
    strIngredient6: String,
    strIngredient7: String,
    strIngredient8: String,
    strIngredient9: String,
    strIngredient10: String,
    strIngredient11: String,
    strIngredient12: String,
    strIngredient13: String,
    strIngredient14: String,
    strIngredient15: String,
    strIngredient16: String,
    strIngredient17: String,
    strIngredient18: String,
    strIngredient19: String,
    strIngredient20: String,
    strMeasure1: String,
    strMeasure2: String,
    strMeasure3: String,
    strMeasure4: String,
    strMeasure5: String,
    strMeasure6: String,
    strMeasure7: String,
    strMeasure8: String,
    strMeasure9: String,
    strMeasure10: String,
    strMeasure11: String,
    strMeasure12: String,
    strMeasure13: String,
    strMeasure14: String,
    strMeasure15: String,
    strMeasure16: String,
    strMeasure17: String,
    strMeasure18: String,
    strMeasure19: String,
    strMeasure20: String,
    strSource: String,
}

#[derive(Deserialize, Debug)]
pub struct OldMeals {
    meals: Vec<OldMeal>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Ingredient {
    ingredient: String,
    measure: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct Meal {
    meal_id: u16,
    meal_name: String,
    category: String,
    area: String,
    inuctions: String,
    meal_thumb: String,
    tags: String,
    youtube: String,
    source: String,
    ingredient_list : Vec<Ingredient>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Meals {
    meals: Vec<Meal>,
}

fn contains(ingr: &String, meas: &String) -> bool {
    ingr != "" || meas != ""
}

fn create_ingredient(ingr: String, meas: String) -> Option<Ingredient> {
    match contains(&ingr, &meas) {
        true => {
            Some(Ingredient{ingredient: ingr, measure: meas})
        },
        false => None,
    }
}

fn create_ingredients(meal: OldMeal) -> Vec<Ingredient> {
    let mut ingredient_list: Vec<Option<Ingredient>> = vec![];
    ingredient_list.push(create_ingredient(meal.strIngredient1, meal.strMeasure1));
    ingredient_list.push(create_ingredient(meal.strIngredient2, meal.strMeasure2));
    ingredient_list.push(create_ingredient(meal.strIngredient3, meal.strMeasure3));
    ingredient_list.push(create_ingredient(meal.strIngredient4, meal.strMeasure4));
    ingredient_list.push(create_ingredient(meal.strIngredient5, meal.strMeasure5));
    ingredient_list.push(create_ingredient(meal.strIngredient6, meal.strMeasure6));
    ingredient_list.push(create_ingredient(meal.strIngredient7, meal.strMeasure7));
    ingredient_list.push(create_ingredient(meal.strIngredient8, meal.strMeasure8));
    ingredient_list.push(create_ingredient(meal.strIngredient9, meal.strMeasure9));
    ingredient_list.push(create_ingredient(meal.strIngredient10, meal.strMeasure10));
    ingredient_list.push(create_ingredient(meal.strIngredient11, meal.strMeasure11));
    ingredient_list.push(create_ingredient(meal.strIngredient12, meal.strMeasure12));
    ingredient_list.push(create_ingredient(meal.strIngredient13, meal.strMeasure13));
    ingredient_list.push(create_ingredient(meal.strIngredient14, meal.strMeasure14));
    ingredient_list.push(create_ingredient(meal.strIngredient15, meal.strMeasure15));
    ingredient_list.push(create_ingredient(meal.strIngredient16, meal.strMeasure16));
    ingredient_list.push(create_ingredient(meal.strIngredient17, meal.strMeasure17));
    ingredient_list.push(create_ingredient(meal.strIngredient18, meal.strMeasure18));
    ingredient_list.push(create_ingredient(meal.strIngredient19, meal.strMeasure19));
    ingredient_list.push(create_ingredient(meal.strIngredient20, meal.strMeasure20));
    let mut ingredients: Vec<Ingredient> = vec![];
    for elem in ingredient_list {
        if elem.is_some(){
            ingredients.push(elem.unwrap());
        }
    }
    ingredients
}

fn main() {
    println!("Starting CulinarWizard!");
    let data = fs::read_to_string("resources/meals.json").expect("File doesn't exist!");
    let old_meals = serde_json::from_str::<OldMeals>(&data).expect("Couldn't parse json!");
    let mut new_meals = Meals::default();
    for old_meal in old_meals.meals {
        let ingredient_list = create_ingredients(old_meal.clone());
        new_meals.meals.push(Meal{
            meal_id: old_meal.idMeal.parse::<u16>().unwrap(),
            meal_name: old_meal.strMeal,
            category: old_meal.strCategory,
            area: old_meal.strArea,
            inuctions: old_meal.strInstructions,
            meal_thumb: old_meal.strMealThumb,
            tags: old_meal.strTags,
            youtube: old_meal.strYoutube,
            ingredient_list,
            source: old_meal.strSource,
        })
    }
    let new_data = serde_json::to_string(&new_meals).expect("Couldn't serialize json!");
    fs::write("resources/meals2.json", new_data).expect("File doesn't exist!");
    println!("Success!");
}

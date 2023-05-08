use std::io::stdout;

use cli_prompts::{display_prompt, Input};

#[derive(Debug)]
enum CarModel {
    Audi,
    BMW,
    Chevrolet
}

fn validation(input: &str) -> Result<u32, String> {
    input.parse::<u32>().map_err(|e| e.to_string())
}

fn car_to_string(car: &CarModel) -> String {
    match car {
        CarModel::Audi => "Audi A3".into(),
        CarModel::BMW => "BMW X5".into(),
        CarModel::Chevrolet => "Chevrolet 11".into()
    }
}

fn main() {
    let desserts = ["Tiramisu", "Cheesecake", "Brownee", "Cookie", "Jelly"];
    let cars = [CarModel::Audi, CarModel::BMW, CarModel::Chevrolet];

    let input_prompt = cli_prompts::Input::new("Enter your name", |s| Ok(s.to_string()))
        .default_value(String::from("John"));
    let confirmation =
        cli_prompts::Confirmation::new("Do you want a cup of coffee?").default_positive(true);
    let dessert_selection = cli_prompts::Selection::new("Your favoite dessert", desserts.into_iter());
    let car_selection = cli_prompts::Selection::new_with_transformation("Your car model", cars.into_iter(), car_to_string);

    let mut stdout = stdout();
    let name = display_prompt(input_prompt, &mut stdout);
    let is_coffee = display_prompt(confirmation, &mut stdout);
    let dessert = display_prompt(dessert_selection, &mut stdout);
    let car = display_prompt(car_selection, &mut stdout);

    println!("Name: {:?}", name);
    println!("Is coffee: {:?}", is_coffee);
    println!("Dessert: {:?}", dessert);
    println!("Car: {:?}", car);
}

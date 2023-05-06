use std::io::stdout;

use cli_prompts::{display_prompt, Input};

fn validation(input: &str) -> Result<u32, String> {
    input.parse::<u32>().map_err(|e| e.to_string())
}

fn main() {
    let input_prompt =
        cli_prompts::Input::new("Enter your name", |s| Ok(s.to_string())).default_value(String::from("John"));
    let confirmation = cli_prompts::Confirmation::new("Do you want a cup of coffee?").default_positive(true);

    let mut stdout = stdout();
    let name = display_prompt(input_prompt, &mut stdout);
    let is_coffee = display_prompt(confirmation, &mut stdout);

    println!("Name: {:?}", name);
    println!("Is coffee: {:?}", is_coffee);
}

use std::io::stdout;

use cli_prompts::{display_prompt, Input};

fn validation(input: &str) -> Result<u32, String> {
    input.parse::<u32>().map_err(|e| e.to_string())
}

fn main() {
    let input_prompt =
        cli_prompts::Input::new("Enter your name", validation).default_value(String::from("John"));

    let mut stdout = stdout();
    match display_prompt(input_prompt, &mut stdout) {
        Ok(result) => println!("Prompt result: {}", result),
        Err(error) => println!("Prompt failed: {:?}", error),
    }
}

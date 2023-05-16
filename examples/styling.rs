use std::io::{stdout, Result};

use cli_prompts::{
    display_prompt,
    style::{Color, Formatting, PromptStyle},
    Input, InputStyle,
};

fn main() -> Result<()> {
    let style = InputStyle::default()
        .label_style(
            PromptStyle::default()
                .prefix("*")
                .prefix_formatting(Formatting::default().foreground_color(Color::Yellow))
                .prompt_formatting(Formatting::default().italic().underline()),
        )
        .input_formatting(Formatting::default().foreground_color(Color::DarkYellow))
        .submitted_formatting(Formatting::default().foreground_color(Color::Yellow));

    let mut stdout = stdout();
    let prompt = Input::new("Enter your name", |s| Ok(s.to_string())).style(style);

    let result = display_prompt(prompt, &mut stdout);
    println!("Name: {:?}", result);

    Ok(())
}

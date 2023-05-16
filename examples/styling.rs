use std::io::{stdout, Result};

use cli_prompts::{
    prompts::{DisplayPrompt, Input, InputStyle},
    style::{Color, Formatting, LabelStyle},
};

fn main() -> Result<()> {
    let style = InputStyle::default()
        .label_style(
            LabelStyle::default()
                .prefix("*")
                .prefix_formatting(Formatting::default().foreground_color(Color::Yellow))
                .prompt_formatting(Formatting::default().italic().underline()),
        )
        .input_formatting(Formatting::default().foreground_color(Color::DarkYellow))
        .submitted_formatting(Formatting::default().foreground_color(Color::Yellow));

    let mut stdout = stdout();
    let prompt = Input::new("Enter your name", |s| Ok(s.to_string())).style(style);

    let result = prompt.display(&mut stdout);
    println!("Name: {:?}", result);

    Ok(())
}

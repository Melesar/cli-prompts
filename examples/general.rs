use cli_promts;

fn main() {
    let mut stdout = std::io::stdout();
    let result = cli_promts::input("Hello")
        .default_value("World".into())
        .esc_interrupts(true)
        .show(&mut stdout);

    let is_confirmed = cli_promts::confirmation()
        .show(&mut stdout, "Do you want a cookie?")
        .unwrap_or(false);

    println!(
        "Input: {}, confirmation: {}",
        result.unwrap_or("Error occured while providing input".into()),
        is_confirmed
    );
}

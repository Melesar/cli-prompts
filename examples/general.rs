use cli_promts;

fn main() {
    let mut stdout = std::io::stdout();
    let result = cli_promts::input("Hello")
        .default_value("World".into())
        .esc_interrupts(true)
        .show(&mut stdout);

    println!(
        "{}",
        result.unwrap_or("Error occured while providing input".into())
    )
}

use cli_prompts;

fn main() -> Result<(), cli_prompts::error::Error> {
    let mut stdout = std::io::stdout();

    let cities = [
        "Warsaw",
        "Berlin",
        "Zurich",
        "Milano",
        "Montreal",
        "New Yourk",
        "Singapoure",
        "Tokio",
        "Sydney",
        "Cairo",
    ]
    .map(|s| s.to_string());

    let name = cli_prompts::input("Hi! Please enter your name").show(&mut stdout)?;
    let city = cli_prompts::select_one("Which city do you live in?", cities.into_iter())
        .show(&mut stdout)?;
    let like_cats = cli_prompts::confirmation().show(&mut stdout, "Do you like cats?");

    Ok(())
}

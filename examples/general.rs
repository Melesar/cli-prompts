use cli_promts;

fn main() -> Result<(), cli_promts::error::Error> {
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

    let name = cli_promts::input("Hi! Please enter your name").show(&mut stdout)?;
    let city = cli_promts::select_one("Which city do you live in?", cities.into_iter())
        .show(&mut stdout)?;
    let like_cats = cli_promts::confirmation().show(&mut stdout, "Do you like cats?");

    Ok(())
}

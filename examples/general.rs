use cli_promts;

fn validation(input: &str) -> Result<(), String> {
    Err(String::from("Invalid input"))
}

fn main() {
    let mut stdout = std::io::stdout();
    let result = cli_promts::input("Hello")
        .default_value("World".into())
        .show(&mut stdout)
        .unwrap();

    println!("{}", result)
}

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("Choice is {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    let choice: Result<MenuChoice, _> = get_choice("mainmenu");
    
    println!("Choice is {:?}", choice);
    
    match choice {
        Ok(choice) => print_choice(&choice),
        Err(e) => println!("Error: {:?}", e),
    }

    pick_choice("start");
    pick_choice("something");
}

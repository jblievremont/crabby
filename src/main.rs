use std::env;

mod greetings;
use greetings::greets;

mod chifoumi;
use chifoumi::play;
use chifoumi::Game;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = args.get(1).expect("Please provide a command in (greet, chifoumi)");

    match cmd.as_str() {
        "greet" => {
            let name = args.get(2).expect("Name is required for greetings");
            println!("{}", greets(&name));
        }
        "chifoumi" => {
            let p_one = args.get(2).expect("Player one is required for chifoumi");
            let p_two = args.get(3).expect("Player two is required for chifoumi");
            // cast to Game enum
            let p_one: Game = Game::try_from(p_one).unwrap();
            let p_two: Game = Game::try_from(p_two).unwrap();

            let result = play(&p_one, &p_two);
            println!("p1 vs p2 : {:?}", result);
        }
        _ => eprintln!("Not supported command"),
    }
}

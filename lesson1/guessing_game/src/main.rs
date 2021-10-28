use std::io::Write;

fn main() {
    let mut state = State::Menu;

    loop {
        let new_state = state.update();

        if let State::Exit = new_state {
            break;
        }
        state = new_state;
    }

    println!("Exit....")
}

enum State {
    Menu,
    Game,
    Exit,
}

impl State {
    pub fn update(&self) -> Self {
        match self {
            Self::Menu => Self::run_menu(),
            Self::Game => Self::run_game(),
            Self::Exit => panic!("Unexpected State::Exit"),
        }
    }

    fn run_menu() -> State {
        println!();
        println!("*** MENU ***");
        println!("1) Start game");
        println!("Other) Exit");
        let choice = Self::get_choice();
        match choice {
            Some(1) => State::Game,
            _ => State::Exit,
        }
    }

    fn get_choice() -> Option<u32> {
        std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        buffer.trim().parse().ok()
    }

    fn run_game() -> State {
        let number: u32 = rand::random();
        let number = number % 100;

        println!();
        println!("*** Game ***");

        loop {
            let choice = Self::get_choice();

            match choice {
                Some(x) if x < number => println!("Too small!"),
                Some(x) if x < 100 && x > number => println!("Too big!"),
                Some(x) if x == number => {
                    println!("Bingo!");
                    return Self::Menu;
                }
                _ => return Self::Menu,
            }
        }

        todo!()
    }
}

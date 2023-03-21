mod scheduler;

use std::{
    io::{self, Write},
    process::exit,
};

#[derive(Debug)]
enum Choice {
    Slot,
    Activity,
    Schedules,
    Quit,
}

fn main() {
    println!("{}", get_name_text());

    print_intro_message();

    let input = get_user_input();

    if let Choice::Quit = input {
        exit(0);
    }
}

fn get_name_text() -> &'static str {
    r###"
    _____        _                _         _                          _   
   / ____|      | |              | |       | |                        | |  
  | (___    ___ | |__    ___   __| | _   _ | |  ___  _ __  _   _  ___ | |_ 
   \___ \  / __|| '_ \  / _ \ / _` || | | || | / _ \| '__|| | | |/ __|| __|
   ____) || (__ | | | ||  __/| (_| || |_| || ||  __/| |   | |_| |\__ \| |_ 
  |_____/  \___||_| |_| \___| \__,_| \__,_||_| \___||_|    \__,_||___/ \__|
"###
}

fn print_intro_message() {
    println!("Welcome to schedulerust!");
    println!(
        "To use schedulerust you'll need to insert all you slots and activities, respectively"
    );
    println!("For example:");
    println!("I have 2 slots for classes everyday, one at 19:00 and one at 20:50");
    println!("So, I will create one slot for each");

    println!("After that, I will create all the activities that I MAY attend, and associate these activities with the slots they will use");
    println!(
        "For example, if I have a Data Structures class at monday 19:00 and thursday 20:50, \
        I will create the Data Structures activity related to the slots monday 19:00 and thursday 20:50 \
        and so on with all the other activities I have"
    );

    println!(
        "IMPORTANT: A slot is just a name, so slots with the same name will be considered the same"
    );
}

fn get_user_input() -> Choice {
    println!("So, what do you want to do?");
    loop {
        println!("[1] - Input slots");
        println!("[1] - Input activities");
        println!("[3] - Get all possible schedules");
        println!("[4] - Quit");
        print!("Type your choice (1, 2 or 3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u8>() {
            Ok(1) => return Choice::Slot,
            Ok(2) => return Choice::Activity,
            Ok(3) => return Choice::Schedules,
            Ok(4) => return Choice::Quit,
            _ => {
                println!("Invalid input");
                continue;
            }
        }
    }
}

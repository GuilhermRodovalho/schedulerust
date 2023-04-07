mod scheduler;

use std::{
    io::{self, Write},
    process::exit,
};

use scheduler::{Activity, Schedule, Slot};

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

    let mut slots = Vec::<Slot>::new();
    let mut activities = Vec::<Activity>::new();

    loop {
        let input = get_user_input();

        match input {
            Choice::Slot => {
                slots.append(&mut input_slots());
            }
            Choice::Activity => {
                activities.append(&mut input_activities(&slots));
            }
            Choice::Schedules => {
                let schedules = get_schedules(&activities, &slots);
                print_schedules(&schedules);
            }
            Choice::Quit => {
                exit(0);
            }
        }
    }
}

// Functions to get user input
fn get_schedules(activities: &Vec<Activity>, slots: &Vec<Slot>) -> Vec<Schedule> {
    clear_screen();

    if activities.is_empty() || slots.is_empty() {
        println!("You need to input activities and slots first!");
        println!("Press enter to continue");
        read_str();
        return Vec::<Schedule>::new();
    }

    // read the length of the schedule from the user
    println!(
        "How many activities do you want in each schedule? (1 to {})",
        activities.len()
    );

    let schedule_length: u8 = read_str().parse().unwrap();

    println!("Calculating schedules...");

    Schedule::get_all_valid_schedules(activities, slots, schedule_length)
}

fn print_schedules(schedules: &Vec<Schedule>) {
    clear_screen();
    println!("You have {} possible schedules", schedules.len());
    for (i, schedule) in schedules.iter().enumerate() {
        println!("[{}] - {}", i + 1, schedule);
    }
    println!("Press enter to continue");
    read_str();
}

fn input_activities(slots: &Vec<Slot>) -> Vec<Activity> {
    clear_screen();
    if slots.is_empty() {
        println!("You need to input slots first!");
        println!("Press enter to continue");
        read_str();
        return Vec::<Activity>::new();
    }
    println!("Activities input!");
    let mut activities = Vec::<Activity>::new();

    loop {
        clear_screen();
        if !activities.is_empty() {
            print_activities("You have the following activities:\n", &activities);
        }
        print_slots("You have the following slots:\n", slots);
        print!("Type the name of the activity (ENTER to quit): ");
        let activity_name = read_str();
        if activity_name.is_empty() {
            break;
        }
        let mut chosen_slots = Vec::<&Slot>::new();
        println!("What slots this activity uses? (choose by the number)");
        println!("When finished, type 0 to quit");
        loop {
            print!("Slot number: ");
            let slot_number = read_str();
            let slot_number = match slot_number.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid input");
                    continue;
                }
            };
            if slot_number == 0 {
                break;
            }
            if slot_number > slots.len() || slot_number < 1 {
                println!("Invalid input");
                continue;
            }
            chosen_slots.push(slots.get(slot_number - 1).unwrap());
        }

        let activity = Activity::new_with_slots(activity_name.as_str(), chosen_slots);
        activities.push(activity);
    }

    activities
}

/// Get user input for slots
fn input_slots() -> Vec<Slot> {
    println!("Slots input");

    let mut slots = Vec::<Slot>::new();

    loop {
        clear_screen();
        println!("Type the names of the slots");
        println!("When finished, type enter to quit");
        if !slots.is_empty() {
            print_slots("You already have the following slots:\n", &slots);
        }
        print!("Name: ");

        let name = read_str();
        if name.is_empty() {
            break;
        }
        let slot = Slot::new(name.as_str());
        slots.push(slot);
    }

    slots
}

fn print_slots(label: &str, slots: &[Slot]) {
    print!("{}", label);
    for (i, slot) in slots.iter().enumerate() {
        println!("[{}] - {:?}", i + 1, slot.name);
    }
}

fn print_activities(label: &str, activities: &[Activity]) {
    print!("{}", label);
    for (i, activity) in activities.iter().enumerate() {
        println!("[{}] - {}", i + 1, activity.name);
        print!("{}", formated_activity_slots(activity));
    }
}

fn formated_activity_slots(activity: &Activity) -> String {
    let mut slots = String::new();
    for slot in activity.slots_to_use.iter() {
        slots.push_str(&format!("\t- {}\n", slot.name));
    }
    slots
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
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
        "To use schedulerust you'll need to insert all your slots and activities, respectively"
    );
    println!("For example:");
    println!("I have 2 slots for classes everyday, one at 19:00 and one at 20:50");
    println!("So, I will create one slot for each class from monday to friday");

    println!("After that, I will create all the activities that I MAY attend, and associate these activities with the slots they will use");
    println!(
        "For example, if I have a Data Structures class at monday 19:00 and thursday 20:50, \
        I will create the Data Structures activity related to the slots monday 19:00 and thursday 20:50 \
        and so on with all the other activities I have"
    );

    println!(
        "IMPORTANT: A slot is just a name, so slots with the same name will be considered to be equal"
    );
}

fn get_user_input() -> Choice {
    println!("So, what do you want to do?");
    loop {
        println!("[1] - Input slots");
        println!("[2] - Input activities");
        println!("[3] - Get all possible schedules");
        println!("[4] - Quit");
        print!("Type your choice (1, 2, 3 or 4): ");

        let input = read_str();

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

fn read_str() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

use std::{result, vec};

#[derive(Debug, Clone)]
enum SlotState {
    Free,
    Full,
}

#[derive(Debug, Clone)]
struct Slot {
    name: String,
    state: SlotState,
}

#[derive(Debug, Clone)]
struct Activity {
    name: String,
    slot_to_use: Vec<Option<Box<Slot>>>,
}

#[derive(Debug, Clone)]
struct Schedule {
    activities: Vec<Activity>,
}

impl Slot {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            state: SlotState::Free,
        }
    }
}

impl Activity {
    // fn new(name: &str) -> Self {
    //     Self {
    //         name: name.into(),
    //         slot_to_use: Vec::new(),
    //     }
    // }

    fn new_with_slots(name: &str, slots: Vec<&Slot>) -> Self {
        Self {
            name: name.into(),
            slot_to_use: slots
                .into_iter()
                .map(|s| Some(Box::new(s.clone())))
                .collect::<Vec<Option<Box<Slot>>>>(),
        }
    }
}

fn main() {
    let slots = [
        Slot::new("seg 19 horas"),
        Slot::new("seg 20 horas"),
        Slot::new("terc 19 horas"),
        Slot::new("terc 20 horas"),
        Slot::new("qua 19 horas"),
        Slot::new("qua 20 horas"),
        Slot::new("qui 19 horas"),
        Slot::new("qui 20 horas"),
        Slot::new("sex 19 horas"),
        Slot::new("sex 20 horas"),
        Slot::new("sabado"),
    ];

    let activities = vec![
        Activity::new_with_slots("Projetos", vec![&slots[0], &slots[1]]),
        Activity::new_with_slots("organizacao e rec da info", vec![&slots[2], &slots[7]]),
        Activity::new_with_slots("gerencia de projetos", vec![&slots[3], &slots[4]]),
        Activity::new_with_slots("Mat Fin", vec![&slots[8], &slots[9]]),
        Activity::new_with_slots("resolucao", vec![&slots[10]]),
    ];

    // let res = associate_slots_to_activities(slots.to_vec(), activities);

    let res = get_activity_permutations(&activities);

    for (i, schedule) in res.iter().enumerate() {
        // print!("----------------------------------------------------\n");
        // println!("Schedule N{}", i + 1);
        // for activity in schedule {
        //     print!("{}, ", activity.name);
        // }
        // println!();
    }

    // println!("{:?}", res);
}

fn permutations(items: &mut Vec<usize>) -> Vec<Vec<usize>> {
    if items.is_empty() {
        vec![vec![]]
    } else {
        let mut result = vec![];
        for i in 0..items.len() {
            let item = items.remove(i);
            let sub_permutations = permutations(items);
            for mut perm in sub_permutations {
                perm.insert(0, item.clone());
                result.push(perm);
            }
            items.insert(i, item);
        }
        result
    }
}

fn get_activity_permutations(activities: &[Activity]) -> Vec<Vec<Activity>> {
    let mut activity_permutations = vec![];
    let mut activity_indexes = (0..activities.len()).collect::<Vec<usize>>();
    let index_permutations = permutations(&mut activity_indexes);

    println!("Permutations {:?}", index_permutations);

    for index_permutation in index_permutations {
        let mut activity_permutation = vec![];
        for activity_index in index_permutation {
            activity_permutation.push(activities[activity_index].clone());
        }
        activity_permutations.push(activity_permutation);
    }
    activity_permutations
}

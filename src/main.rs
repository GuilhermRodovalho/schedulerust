use std::vec;

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

    let res = associate_slots_to_activities(slots.to_vec(), activities);

    println!("{:?}", res);
}

/// Returns all possible combinations of schedules that can be produced with the given slots and activities.
///
/// This is a recursive function that associates slots with activities, such that each activity is assigned
/// to one or more slots, and no two activities are assigned to the same slot. The result is a vector of activities
/// where each activity has a `slot_to_use` field indicating which slots it is associated with.
///
/// # Arguments
///
/// * `slots` - A vector of `Slot` objects representing the available time slots.
/// * `activities` - A vector of `Activity` objects representing the activities to be scheduled.
///
/// # Returns
///
/// A vector of `Activity` objects, each with a `slot_to_use` field indicating which slots it is associated with.
fn associate_slots_to_activities(slots: Vec<Slot>, activities: Vec<Activity>) -> Vec<Activity> {
    let mut slots = slots
        .into_iter()
        .map(|slot| Some(Box::new(slot)))
        .collect::<Vec<Option<Box<Slot>>>>();

    let result = associate_slots_to_activities_recursive(&mut slots, activities);

    result
}

fn associate_slots_to_activities_recursive(
    slots: &mut Vec<Option<Box<Slot>>>,
    activities: Vec<Activity>,
) -> Vec<Activity> {
    if slots.is_empty() || activities.is_empty() {
        return [].to_vec();
    }

    // traverse all the activities, and remove the respective slots and the activity that
    // will use those slots

    vec![]
}

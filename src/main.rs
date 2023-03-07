use std::{fmt::Debug, vec};

#[derive(Debug, Clone, Hash)]
struct Slot {
    name: String,
}

#[derive(Debug, Clone, Hash)]
struct Activity {
    name: String,
    slots_to_use: Vec<Slot>,
}

#[derive(Debug, Clone, Hash)]
struct Schedule {
    activities: Vec<Activity>,
}

impl Slot {
    fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl PartialEq for Slot {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialEq for Activity {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        for this_slot in &self.slots_to_use {
            if !other.slots_to_use.contains(&this_slot) {
                return false;
            }
        }

        for other_slot in &other.slots_to_use {
            if !self.slots_to_use.contains(&other_slot) {
                return false;
            }
        }

        true
    }
}

impl PartialEq for Schedule {
    fn eq(&self, other: &Self) -> bool {
        for activity in &self.activities {
            if !other.activities.contains(activity) {
                return false;
            }
        }

        for activity in &other.activities {
            if !self.activities.contains(activity) {
                return false;
            }
        }

        true
    }
}

impl Eq for Schedule {}

impl Activity {
    fn new_with_slots(name: &str, slots: Vec<&Slot>) -> Self {
        Self {
            name: name.into(),
            slots_to_use: slots.into_iter().cloned().collect(),
        }
    }

    fn can_be_allocated_in(&self, slots: &[Slot]) -> bool {
        self.slots_to_use.iter().all(|slot| slots.contains(slot))
    }
}

impl Schedule {
    fn get_possible_schedules(activities: &[Activity], num_of_activities: i8) -> Vec<Self> {
        let mut activity_permutations = vec![];
        let mut activity_indexes = (0..activities.len()).collect::<Vec<usize>>();
        let index_permutations = permutations(&mut activity_indexes);

        for index_permutation in index_permutations {
            let mut activity_permutation = vec![];
            let mut atv_count = 0;
            for activity_index in index_permutation {
                activity_permutation.push(activities[activity_index].clone());
                atv_count += 1;
                if atv_count == num_of_activities {
                    break;
                }
            }
            activity_permutations.push(activity_permutation);
        }
        let mut all_schedules: Vec<Self> = Vec::new();

        for activities in activity_permutations {
            let schedule = Self { activities };
            all_schedules.push(schedule);
        }

        all_schedules
    }

    fn pretty_print(&self) {
        println!("--------------------------------");
        for a in &self.activities {
            println!("{}", a.name);
        }
    }
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Schedule")?;
        for activity in &self.activities {
            writeln!(f, "\t{}\n", activity.name)?;
        }
        Ok(())
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
        Activity::new_with_slots("PDS1", vec![&slots[8], &slots[5]]),
        Activity::new_with_slots("resolucao", vec![&slots[10]]),
    ];

    let _res = get_all_valid_schedules(&activities, &slots, 4);
}

fn get_all_valid_schedules(
    activities: &[Activity],
    slots: &[Slot],
    num_of_activities: i8,
) -> Vec<Schedule> {
    let possible_schedules = Schedule::get_possible_schedules(activities, num_of_activities);

    println!("I hope at least here we had a 2 and 3");
    for s in &possible_schedules {
        s.pretty_print();
    }

    let valid_schedules =
        filter_valid_schedules(possible_schedules, slots.to_vec(), num_of_activities);

    println!("Here we still had a 2 and 3");
    for s in &valid_schedules {
        s.pretty_print();
    }

    let filtered_schedules = filter_identical_schedules(valid_schedules);

    println!(
        "So, finally, we got {} different schedules for you to choose",
        filtered_schedules.len()
    );

    for s in &filtered_schedules {
        println!("Schedule: ");
        for a in &s.activities {
            println!("\t{}", a.name);
        }
    }

    filtered_schedules
}

fn filter_identical_schedules(possible_schedules: Vec<Schedule>) -> Vec<Schedule> {
    let mut aux_possible: Vec<Option<Schedule>> = possible_schedules
        .iter()
        .map(|schedule| Some(schedule.clone()))
        .collect();

    let schedules_length = possible_schedules.len();
    for i in 0..schedules_length {
        for j in i + 1..schedules_length {
            if possible_schedules.get(i) == possible_schedules.get(j) {
                aux_possible[i] = None;
            }
        }
    }

    aux_possible
        .into_iter()
        .filter(|schd| schd.is_some())
        .map(|schd| schd.unwrap())
        .collect()
}

fn filter_valid_schedules(
    all_schedules: Vec<Schedule>,
    slots: Vec<Slot>,
    num_of_activities: i8,
) -> Vec<Schedule> {
    let mut result_schedules = Vec::new();
    for schedule in all_schedules {
        let new_slots = slots.clone();
        if filter_schedule_with_slots(&schedule, new_slots, num_of_activities) {
            result_schedules.push(schedule);
        }
    }

    result_schedules
}

fn filter_schedule_with_slots(
    schedule: &Schedule,
    mut slots: Vec<Slot>,
    mut num_of_activities: i8,
) -> bool {
    for activity in &schedule.activities {
        if num_of_activities == 0 {
            return true;
        }

        let size_before = slots.len();

        if activity.can_be_allocated_in(&slots) {
            slots = slots
                .into_iter()
                .filter(|slot| {
                    if activity.slots_to_use.contains(slot) {
                        return false;
                    }
                    true
                })
                .collect();
        }

        // means it wasn't able to find a slot for this activity
        if slots.len() == size_before {
            return false;
        }

        num_of_activities -= 1;
    }

    true
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
                perm.insert(0, item);
                result.push(perm);
            }
            items.insert(i, item);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_slot() {
        let slot = Slot::new("teste");
        let my_slot = Slot {
            name: "teste".into(),
        };
        assert_eq!(slot.name, my_slot.name);
    }

    fn create_default_slots_with_equals() -> [Slot; 3] {
        [Slot::new("teste"), Slot::new("teste1"), Slot::new("teste1")]
    }

    fn create_default_slots() -> [Slot; 3] {
        [Slot::new("teste"), Slot::new("teste1"), Slot::new("teste2")]
    }

    #[test]
    fn test_slot_equals() {
        let slots = create_default_slots_with_equals();

        assert_eq!(slots[1], slots[2]);
        assert_ne!(slots[0], slots[2]);
    }

    #[test]
    fn test_activity_new_with_slots() {
        let slot1 = Slot::new("teste");
        let slot2 = Slot::new("teste1");
        let slot3 = Slot::new("teste1");

        let activity = Activity::new_with_slots("nova atividade", vec![&slot1, &slot2, &slot3]);

        assert!(activity.slots_to_use.contains(&slot1));
        assert!(activity.slots_to_use.contains(&slot2));
        assert!(activity.slots_to_use.contains(&slot3));
    }

    #[test]
    fn test_activity_can_be_allocated_in() {
        let slots = create_default_slots_with_equals();
        let new_slots = [Slot::new("um outro slot")];

        let atv = Activity::new_with_slots("teste", vec![&slots[0]]);
        println!("{:?}", atv);
        assert!(atv.can_be_allocated_in(&slots));
        assert!(!atv.can_be_allocated_in(&new_slots));
    }

    #[test]
    fn test_permutations() {
        let mut nums: Vec<usize> = vec![1, 2, 3];

        let perms = permutations(&mut nums);
        assert!(perms.contains(&vec![2, 1, 3]));
        assert!(perms.contains(&vec![3, 1, 2]));
        assert!(perms.contains(&vec![1, 3, 2]));
        assert!(perms.contains(&vec![3, 2, 1]));
    }

    #[test]
    fn test_equals_schedules() {
        let activities = create_default_activities();

        let schdl1 = Schedule {
            activities: vec![
                activities.get(1).unwrap().clone(),
                activities.get(0).unwrap().clone(),
            ],
        };
        let schdl2 = Schedule {
            activities: vec![
                activities.get(1).unwrap().clone(),
                activities.get(0).unwrap().clone(),
            ],
        };
        let schdl3 = Schedule {
            activities: vec![
                activities.get(2).unwrap().clone(),
                activities.get(1).unwrap().clone(),
            ],
        };

        assert_eq!(schdl1, schdl2);
        assert_ne!(schdl1, schdl3);
    }

    fn create_default_activities() -> Vec<Activity> {
        let slots = create_default_slots();

        let mut activities = Vec::<Activity>::new();
        activities.push(Activity {
            name: "atv1".into(),
            slots_to_use: vec![slots[0].clone()],
        });
        activities.push(Activity {
            name: "atv2".into(),
            slots_to_use: vec![slots[2].clone()],
        });
        activities.push(Activity {
            name: "atv3".into(),
            slots_to_use: vec![slots[1].clone()],
        });

        activities
    }

    #[test]
    fn test_schedule_get_possible_schedules() {
        let atvs = create_default_activities();

        let schedules = Schedule::get_possible_schedules(&atvs, 2);

        assert!(schedules.contains(&Schedule {
            activities: vec![atvs.get(0).unwrap().clone(), atvs.get(1).unwrap().clone(),]
        }));
        assert!(schedules.contains(&Schedule {
            activities: vec![atvs.get(1).unwrap().clone(), atvs.get(2).unwrap().clone(),]
        }));
    }

    #[test]
    fn test_filter_schedule_with_slots() {
        let atvs = create_default_activities();
        let slots = create_default_slots();

        let schd = Schedule {
            activities: vec![atvs[0].clone()],
        };

        assert!(!filter_schedule_with_slots(
            &schd,
            vec![slots[1].clone()],
            1
        ));
        assert!(filter_schedule_with_slots(&schd, vec![slots[0].clone()], 1))
    }

    #[test]
    fn test_get_all_schedules() {
        let activities = create_default_activities();
        let slots = create_default_slots();

        let possible_schedules = get_all_valid_schedules(&activities, &slots, 2);

        assert!(possible_schedules.contains(&Schedule {
            activities: vec![
                activities.get(0).unwrap().clone(),
                activities.get(2).unwrap().clone(),
            ],
        }));
        assert!(possible_schedules.contains(&Schedule {
            activities: vec![
                activities.get(0).unwrap().clone(),
                activities.get(1).unwrap().clone(),
            ],
        }));
        assert!(possible_schedules.contains(&Schedule {
            activities: vec![
                activities.get(1).unwrap().clone(),
                activities.get(2).unwrap().clone(),
            ],
        }));
    }
}

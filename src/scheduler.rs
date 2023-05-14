use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Slot {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Activity {
    pub name: String,
    pub slots_to_use: Vec<Slot>,
}

#[derive(Debug, Clone)]
pub struct Schedule {
    activities: Vec<Activity>,
}

impl Slot {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl PartialEq for Slot {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Slot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Activity {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        for this_slot in &self.slots_to_use {
            if !other.slots_to_use.contains(this_slot) {
                return false;
            }
        }

        for other_slot in &other.slots_to_use {
            if !self.slots_to_use.contains(other_slot) {
                return false;
            }
        }

        true
    }
}

impl Eq for Activity {}

impl Hash for Activity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let mut names = Vec::new();
        for slot in &self.slots_to_use {
            names.push(slot.name.clone());
        }
        names.sort();

        names.hash(state);
    }
}

impl PartialOrd for Activity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Activity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::{Equal, Greater, Less};

        match &self.name.cmp(&other.name) {
            std::cmp::Ordering::Less => Less,
            std::cmp::Ordering::Equal => Equal,
            std::cmp::Ordering::Greater => Greater,
        }
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

impl Hash for Schedule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // let activity_names = Vec::new();
        let mut this_sorted_atv = self.activities.clone();
        this_sorted_atv.sort();
        for activity in this_sorted_atv {
            activity.hash(state);
        }
    }
}

impl Activity {
    pub fn new_with_slots(name: &str, slots: Vec<&Slot>) -> Self {
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
    fn get_possible_schedules(activities: &[Activity], num_of_activities: u8) -> Vec<Self> {
        let activity_permutations = activities.iter().permutations(num_of_activities as usize);

        let all_schedules: Vec<Self> = activity_permutations
            .map(|permutation| {
                let activities: Vec<Activity> = permutation.into_iter().cloned().collect();
                Schedule { activities }
            })
            .collect();

        all_schedules
    }

    /// Get all valid schedules for the given activities and slots with the given number of activities
    pub fn get_all_valid_schedules(
        activities: &[Activity],
        slots: &[Slot],
        num_of_activities: u8,
    ) -> Vec<Self> {
        let possible_schedules = Self::get_possible_schedules(activities, num_of_activities);

        let valid_schedules = Self::filter_valid_schedules(possible_schedules, slots);

        Self::filter_identical_schedules(valid_schedules)
    }

    /// Filter out schedules that cannot be allocated in the given slots
    fn filter_valid_schedules(all_schedules: Vec<Schedule>, slots: &[Slot]) -> Vec<Schedule> {
        let result_schedules = all_schedules
            .into_iter()
            .filter(|schedule| {
                let mut slots = slots.to_owned();
                filter_schedule_with_slots(schedule, &mut slots)
            })
            .collect();

        result_schedules
    }

    /// Filter out identical schedules by using a HashSet to remove duplicates
    fn filter_identical_schedules(possible_schedules: Vec<Self>) -> Vec<Self> {
        let mut unique_schedules: HashSet<Self> = HashSet::new();
        for schedule in possible_schedules {
            unique_schedules.insert(schedule);
        }
        unique_schedules.into_iter().collect()
    }
}

impl std::fmt::Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Schedule")?;
        for activity in &self.activities {
            writeln!(f, "\t{}", activity.name)?;
        }
        Ok(())
    }
}

fn filter_schedule_with_slots(schedule: &Schedule, slots: &mut Vec<Slot>) -> bool {
    for activity in &schedule.activities {
        let size_before = slots.len();

        if activity.can_be_allocated_in(&slots) {
            slots.retain(|slot| {
                if activity.slots_to_use.contains(slot) {
                    return false;
                }
                true
            });
        }

        // means it wasn't able to find a slot for this activity
        if slots.len() == size_before {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;

    use super::*;

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

    fn create_equal_activities() -> [Activity; 2] {
        let slots = create_default_slots();

        [
            Activity {
                name: "atv1".into(),
                slots_to_use: vec![slots[1].clone(), slots[0].clone()],
            },
            Activity {
                name: "atv1".into(),
                slots_to_use: vec![slots[0].clone(), slots[1].clone()],
            },
        ]
    }
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
        assert!(atv.can_be_allocated_in(&slots));
        assert!(!atv.can_be_allocated_in(&new_slots));
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

        let mut slots1 = vec![slots[1].clone()];
        assert!(!filter_schedule_with_slots(&schd, &mut slots1));

        let mut slots2 = vec![slots[0].clone()];
        assert!(filter_schedule_with_slots(&schd, &mut slots2));
    }

    #[test]
    fn test_get_all_schedules() {
        let activities = create_default_activities();
        let slots = create_default_slots();

        let possible_schedules = Schedule::get_all_valid_schedules(&activities, &slots, 2);

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

    #[test]
    fn test_activiy_hash() {
        let activity = create_equal_activities();

        let mut hasher = DefaultHasher::new();
        assert_eq!(activity[0].hash(&mut hasher), activity[1].hash(&mut hasher));
    }

    #[test]
    fn test_schedule_has() {
        let activities = create_default_activities();

        let schedule1 = Schedule {
            activities: activities.clone(),
        };
        let schedule2 = Schedule {
            activities: activities.clone(),
        };

        let mut hasher = DefaultHasher::new();
        assert_eq!(schedule1.hash(&mut hasher), schedule2.hash(&mut hasher));
    }
}

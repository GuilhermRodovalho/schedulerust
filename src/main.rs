mod scheduler;

use scheduler::{get_all_valid_schedules, Activity, Slot};

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

    let res = get_all_valid_schedules(&activities, &slots, 4);

    println!("{:?}", res.len());
}

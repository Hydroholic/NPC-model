pub mod ports;
pub mod domain;

use crate::domain::npc;
use crate::ports::time_giver;


struct TimeFake{
    time: time_giver::TimeState
}

impl time_giver::TimeGiver for TimeFake{
    fn give_time(&self) -> &time_giver::TimeState {
        &self.time
    }
}

fn main() {

    let default_time_state = time_giver::TimeState::Night;
    let default_time_giver = TimeFake { time: default_time_state };
    let default_time_giver_box = Box::new(&default_time_giver);
    let npc = npc::NPC::new("Georges", default_time_giver).expect("Valid NPC");
    // let npc = npc::NPC { name: "Georges", time_giver: default_time_giver_box};  // Access to default contructor not allowed
    println!("{}", npc.greet_player("Alexandre"));
}

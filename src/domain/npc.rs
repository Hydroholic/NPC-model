// use std::time::{Duration, SystemTime};

/*impl Time for NPC {

    fn give_time() -> SystemTime {
        SystemTime::now()
    }
}*/


use crate::ports::time_giver::{TimeGiver, TimeState};
use npc::*;


mod npc {
    use super::*;
    pub struct NPC {
        name: &'static str,
        time_giver: Box<dyn TimeGiver>
    }

    impl NPC {
        pub fn new(
            name: &'static str,
            time_giver: Box<dyn TimeGiver>
        ) -> NPC {
            return NPC { name: name, time_giver: time_giver }
        }

        pub fn greet_player(&self, name: &str) -> String {
            let greeting = String::from("Hello, ");
            greeting + name + "! My name is " + self.name
        }

        pub fn give_mood(&self) -> Mood {
            let time_state = self.time_giver.give_time();
            match time_state {
                TimeState::Morning => Mood::Angry,
                _ => Mood::Fine,
            }
        }

        pub fn get_name(&self) -> &str {
            self.name
        }
    }
}


#[derive(Debug, PartialEq)]
enum Mood {
    Fine, Angry, Sad, Overjoyed
}


#[cfg(test)]
mod tests {
    use super::*;  // importer toutes les fonctions du fichier

    struct TimeFake{
        time: TimeState
    }
    
    impl TimeGiver for TimeFake{
        fn give_time(&self) -> &TimeState {
            &self.time
        }
    }

    fn get_default_time_giver() -> impl TimeGiver {
        let default_time_state = TimeState::Night;
        let default_time_giver = TimeFake { time: default_time_state };
        return default_time_giver
    }

    fn get_default_npc() -> NPC {
        let time_giver_box = Box::new(get_default_time_giver());
        let npc = NPC::new("Georges", time_giver_box);
        return npc
    }

    #[test]
    fn test_npc_has_name() {
        let npc: NPC = get_default_npc();
        assert_eq!(npc.get_name(), "Georges");
    }

    #[test]
    fn test_npc_greets_player() {
        let npc: NPC = get_default_npc();
        let player_name: String = String::from("Corentin");
        assert_eq!(npc.greet_player(&player_name), "Hello, Corentin! My name is Georges");
    }

    #[test]
    fn test_npc_is_ok() {
        let npc: NPC = get_default_npc();
        assert_eq!(npc.give_mood(), Mood::Fine);
    }

    #[test]
    fn test_npc_is_angry_morning() {
        let default_time_state = TimeState::Morning;
        let default_time_giver = TimeFake { time: default_time_state };
        let time_giver_box = Box::new(default_time_giver);
        let npc = NPC::new("Georges", time_giver_box);
        assert_eq!(npc.give_mood(), Mood::Angry);
    }
}
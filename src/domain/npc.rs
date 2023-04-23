// use std::time::{Duration, SystemTime};

/*impl Time for NPC {

    fn give_time() -> SystemTime {
        SystemTime::now()
    }
}*/

trait TimeGiver {
    fn give_time(&self) -> &TimeState;
}

struct NPC {
    name: &'static str,
    time_giver: Option<Box<dyn TimeGiver>> //dyn Option<TimeGiver>
}

impl NPC {
    pub fn greet_player(&self, name: &str) -> String {
        let greeting = String::from("Hello, ");
        greeting + name + "! My name is " + self.name
    }

    pub fn give_mood(&self) -> Mood {
        let time_state = match &self.time_giver {
            Some(time_giver) => time_giver.give_time(),
            None => panic!("Should not access time_giver")
        };
        if *time_state == TimeState::Morning {
            Mood::Angry
        }
        else {
            Mood::Fine
        }
    }
}

struct TimeBidon{
    time: TimeState
}

impl TimeGiver for TimeBidon{
    fn give_time(&self) -> &TimeState {
        &self.time
    }
}

#[derive(Debug, PartialEq)]
enum Mood {
    Fine, Angry, Sad, Overjoyed
}

#[derive(Debug, PartialEq)]
enum TimeState {
    Morning, Night
}


#[cfg(test)]
mod tests {
    use super::*; //importer toutes les fonctions du fichier

    #[test]
    fn test_npc_has_name() {
        let npc: NPC = NPC { name: "Georges", time_giver: None};
        assert_eq!(npc.name, "Georges");
    }

    #[test]
    fn test_npc_greets_player() {
        let npc: NPC = NPC { name: "Georges", time_giver: None };
        let player_name: String = String::from("Corentin");
        assert_eq!(npc.greet_player(&player_name), "Hello, Corentin! My name is Georges");
    }

    #[test]
    fn test_npc_is_ok() {
        let npc: NPC = NPC { name: "Georges", time_giver: None };
        assert_eq!(npc.give_mood(), Mood::Fine);
    }

    #[test]
    fn test_npc_is_angry_morning() {
        let npc: NPC = NPC { name: "Georges", time_giver: None };
        assert_eq!(npc.give_mood(), Mood::Angry);
    }
}
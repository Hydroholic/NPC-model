use std::time::{Duration, SystemTime};

trait TimeGiver {
    fn give_time(&self) -> TimeState;
}

struct NPC {
    name: String,
    current_time: dyn Option<TimeGiver>
}

impl NPC {
    pub fn greet_player(&self, name: &String) -> String {
        "Hello, ".to_owned() + name + "!"
    }

    pub fn give_mood(&self) -> Mood {
        let time_state = self.current_time.give_time();
        if(time_state == TimeState::Morning) {
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
    fn give_time(&self) -> TimeState {
        self.time
    }
}
/*impl Time for NPC {

    fn give_time() -> SystemTime {
        SystemTime::now()
    }
}*/

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
        let npc: NPC = NPC { name: String::from("George") };
        assert_eq!(npc.name, "George");
    }

    #[test]
    fn test_npc_greets_player() {
        let npc: NPC = NPC { name: String::from("George") };
        let player_name: String = String::from("Corentin");
        assert_eq!(npc.greet_player(&player_name), "Hello, Corentin!");
    }

    #[test]
    fn test_npc_is_ok() {
        let npc: NPC = NPC { name: String::from("George") };
        assert_eq!(npc.give_mood(), Mood::Fine);
    }

    #[test]
    fn test_npc_is_angry_morning() {
        let npc: NPC = NPC { name: String::from("George") };
        assert_eq!(npc.give_mood(), Mood::Angry);
    }
}
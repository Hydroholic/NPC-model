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
    time_giver: Box<dyn TimeGiver>
}

impl NPC {
    pub fn greet_player(&self, name: &str) -> String {
        let greeting = String::from("Hello, ");
        greeting + name + "! My name is " + self.name
    }

    pub fn give_mood(&self) -> Mood {
        let time_state = self.time_giver.give_time();
        if *time_state == TimeState::Morning {
            Mood::Angry
        }
        else {
            Mood::Fine
        }
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

    struct TimeBidon{
        time: TimeState
    }
    
    impl TimeGiver for TimeBidon{
        fn give_time(&self) -> &TimeState {
            &self.time
        }
    }

    fn get_default_time_giver() -> impl TimeGiver {
        let default_time_state = TimeState::Morning;
        let default_time_giver = TimeBidon { time: default_time_state };
        return default_time_giver
    }

    fn get_default_npc() -> NPC {
        let time_giver_box = Box::new(get_default_time_giver());
        let npc = NPC { name: "Georges", time_giver: time_giver_box };
        return npc
    }

    #[test]
    fn test_npc_has_name() {
        let npc: NPC = get_default_npc();
        assert_eq!(npc.name, "Georges");
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
        let npc: NPC = get_default_npc();
        assert_eq!(npc.give_mood(), Mood::Angry);
    }
}
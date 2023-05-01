#[allow(dead_code)]

use crate::ports::time_giver::{TimeGiver, TimeState};


pub struct NPCDialog {}

#[derive(Debug, PartialEq)]
pub struct InvalidNameError;

#[derive(Debug, PartialEq)]
pub struct Name { name: String }

impl Name {
    pub fn from(name: &str) -> Result<Name, InvalidNameError> {
        let name = name.to_string();
        match name.chars().all(|c| matches!(c, 'A'..='Z' | 'a'..='z')) {
            true => Ok(Self { name }),
            false => Err(InvalidNameError)
        }
    }

    pub fn to_string(&self) -> &String {
        return &self.name
    }
}

pub struct NPC {
    name: Name,
    time_giver: Box<dyn TimeGiver>,
    // state_of_mind: StateOfMind,
}

impl NPC {
    pub fn new(name: Name, time_giver: impl TimeGiver + 'static ) -> Self {
        let time_giver = Box::new(time_giver);
        Self { name, time_giver }
    }

    pub fn greet_player(&self, name: &str) -> String {
        format!("Hello, {}! My name is {}", name, self.name.to_string())
    }

    pub fn give_mood(&self) -> Mood {
        let time_state = self.time_giver.give_time();
        match time_state {
            TimeState::Morning => Mood::Angry,
            _ => Mood::Fine,
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}

impl PartialEq for NPC {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::fmt::Debug for NPC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NPC")
            .field("Name", &self.name.to_string())
            .finish()
    }
}


#[derive(Debug, PartialEq)]
pub enum Mood {
    Fine, Angry, Sad, Overjoyed
}

pub enum StateOfMind {
    Optimistic,
    Pessimistic,
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

    fn get_default_name() -> Name {
        Name::from("Georges").expect("Default name should be valid.")
    }

    fn get_default_npc() -> NPC {
        let time_giver_box = get_default_time_giver();
        let name = get_default_name();
        NPC::new(name, time_giver_box)
    }

    #[test]
    fn test_npc_has_name() {
        let npc: NPC = get_default_npc();
        assert_eq!(*npc.name(), Name::from("Georges").unwrap());
    }

    #[test]
    fn test_npc_greets_player() {
        let npc: NPC = get_default_npc();
        let player_name: String = String::from("Corentin");
        assert_eq!(npc.greet_player(&player_name), "Hello, Corentin! My name is Georges");
    }

    #[test]
    fn test_name_must_be_valid() {
        // let time_giver_box = get_default_time_giver();
        // let npc = NPC::new("xXSuperGamerDu92Xx", time_giver_box);
        // assert_eq!(npc, Err(NPCError::CreationError));
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
        let default_name = get_default_name();
        let time_giver_box = default_time_giver;
        let npc = NPC::new(default_name, time_giver_box);
        assert_eq!(npc.give_mood(), Mood::Angry);
    }
}
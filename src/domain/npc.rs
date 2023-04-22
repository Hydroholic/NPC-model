struct NPC {
    name: String,
}

impl NPC {
    pub fn greet_player(&self, name: &String) -> String {
        "Hello, ".to_owned() + name + "!"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npc_has_name() {
        let npc: NPC = NPC { name: String::from("George") };
        assert_eq!(npc.name, "George");
    }

    #[test]
    fn test_npc_greets_player() {
        let npc: NPC = NPC { name: String::from("George") };
        let player_name = String::from("Corentin");
        assert_eq!(npc.greet_player(&player_name), "Hello, Corentin!");
    }
}
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Player {
    id: u64,
    first_name: String,
    last_name: String,
    country: String
}

impl Player {
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn get_last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn get_country(&self) -> String {
        self.country.clone()
    }
}
pub fn generate_players() -> Vec<Player> {
    let mut players = Vec::new();

    players.push(Player {
        id: 1,
        first_name: "Scottie".to_string(),
        last_name: "Scheffler".to_string(),
        country: "USA".to_string()
    });

    players.push(Player {
        id: 2,
        first_name: "Tiger".to_string(),
        last_name: "Woods".to_string(),
        country: "USA".to_string()
    });

    players.push(Player {
        id: 3,
        first_name: "Rory".to_string(),
        last_name: "McIlroy".to_string(),
        country: "USA".to_string()
    });

    players.push(Player {
        id: 4,
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        country: "NIR".to_string()
    });

    players.push(Player {
        id: 5,
        first_name: "Ludvig".to_string(),
        last_name: "Åberg".to_string(),
        country: "SWE".to_string()
    });

    players
}
use std::collections::HashMap;
use crate::mcsr::*;

pub struct SeedType {
    pub(crate) code: &'static str,
    pub(crate) name: &'static str,
}

pub static OW_SEEDS: &[SeedType] = &[
    SeedType { code: "SHIPWRECK", name: "Shipwreck" },
    SeedType { code: "RUINED_PORTAL", name: "Ruined Portal" },
    SeedType { code: "BURIED_TREASURE", name: "Buried Treasure" },
    SeedType { code: "DESERT_TEMPLE", name: "Desert Temple" },
    SeedType { code: "VILLAGE", name: "Village" },
];

pub static NETH_SEEDS: &[SeedType] = &[
    SeedType { code: "HOUSING", name: "Housing Bastion" },
    SeedType { code: "STABLES", name: "Stables Bastion" },
    SeedType { code: "TREASURE", name: "Treasure Bastion" },
    SeedType { code: "BRIDGE", name: "Bridge Bastion" },
];

pub fn get_ow_seed(seed: String) -> Option<&'static str> {
    OW_SEEDS.iter()
    .find(|s| s.code == seed)
    .map(|s| s.name)
}
pub fn get_neth_seed(seed: String) -> Option<&'static str> {
    NETH_SEEDS.iter()
    .find(|s| s.code == seed)
    .map(|s| s.name)
}

pub fn determine_winner(player_list: Vec<match_history::Player>, uuid: Option<String>) -> String {
    let mut players = HashMap::new();
    for p in player_list {
        players.insert(p.uuid, p.nickname);
    }
    let winner: String;
    winner = match uuid {
        None => "Draw".to_string(),
        Some(winner_uuid) => {
            players
                .get(&winner_uuid)
                .cloned()
                .unwrap()
        }
    };
    return winner
}
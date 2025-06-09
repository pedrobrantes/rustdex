use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Sprites {
    pub front_default: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub types: Vec<TypeEntry>,
    pub abilities: Vec<AbilityEntry>,
    pub sprites: Sprites,
}

#[derive(Deserialize, Debug)]
pub struct TypeEntry {
    #[serde(rename = "type")]
    pub type_info: Type,
}

#[derive(Deserialize, Debug)]
pub struct Type {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct AbilityEntry {
    pub ability: Ability,
}

#[derive(Deserialize, Debug)]
pub struct Ability {
    pub name: String,
}

//pub(crate) mod troops;
//pub use self::troops::*;
use crate::space::Planet;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Faction {
    Rebels,
    Imperials,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Weapon {
    Melee,
    Ranged,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Character {
    pub name: String,
    pub home_planet: Option<Planet>,
    pub weapon: Option<Weapon>,
    pub dmg: Option<f32>,
}

impl Character {
    pub fn new(
        name: String,
        home_planet: Option<Planet>,
        weapon: Option<Weapon>,
        dmg: Option<f32>,
    ) -> Self {
        Character {
            name,
            home_planet,
            weapon,
            dmg,
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            name: "Unknown".to_string(),
            home_planet: None,
            weapon: None,
            dmg: None,
        }
    }
}

// impl Shoot for Character {
//     fn shoot<T>(&self, other: &T) {
//         println!("\n{:?} (of {:?}) shot at {:?} (of {:?})!",
//                  self.name,
//                  self.home_planet,
//                  other.name,
//                  other.home_planet
//         );
//     }
// }

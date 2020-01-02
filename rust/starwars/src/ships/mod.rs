pub(crate) mod ships;

pub use self::ships::{Ship, Ships};
use crate::troops::Character;
use s2::r3::vector::Vector as r3_vec;

impl Ship {
    pub fn new(
        pilot: Option<Character>,
        kind: Option<Ships>,
        kills: i32,
        fuel: i32,
        health: i32,
        in_flight: i8,
        grounded: i8,
        rebel_scum: i8,
        dmg: f32,
        loc: r3_vec,
    ) -> Self {
        Ship {
            pilot,
            kind,
            kills,
            fuel,
            health,
            in_flight,
            grounded,
            rebel_scum,
            dmg,
            loc,
        }
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            pilot: None,
            kind: None,
            kills: -1,
            fuel: -1,
            health: -1,
            in_flight: -1,
            grounded: -1,
            rebel_scum: -1,
            dmg: -1.0,
            loc: r3_vec {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

pub trait Shoot {
    fn shoot(&self, other: Ship) -> String;
}

impl Shoot for Ship {
    fn shoot(&self, other: Ship) -> String {
        let msg = if let Some(c) = other.pilot {
            // && let Some(x) = other.kind // experimental
            format!(
                "{:?} ({:?}) shot at {:?} ({:?})!",
                self.pilot, self.kind, c.name, other.kind
            )
        //todo: other.health - self.dmg
        } else {
            format!("Can't shoot!")
        };

        msg
    }
}

#[allow(dead_code)]
pub fn travel_distance(ship: &Ship) -> i32 {
    match ship.kind {
        Some(Ships::TIEBomber(..)) | Some(Ships::TIEFighter(..)) => 1 * ship.fuel,
        Some(Ships::XWing(..)) | Some(Ships::YWing(..)) => 10 * ship.fuel,
        None => 0,
    }
}

//todo: figure out Xed methodology
//tldr: i want to impl Shoot for both Ships and Characters (maybe?)
//pub trait Xed {}
//
//impl Xed for Ship {}
//
//impl Xed for Character {}
//
//impl<T> Shoot for T
//    where
//        T: Xed,
//{
//    fn shoot<T>(&self, other: &T) {
//        println!("\n{:?} ({:?}) shot at {:?} ({:?})!",
//                 self.pilot,
//                 self.kind,
//                 other.pilot,
//                 other.kind
//        );
//    }
//}

use rand::prelude::*;
use cgmath::prelude::*;

#[derive(Debug, Clone)]
struct Location {
    x: f32, 
    y: f32, 
    z: f32,
}

#[derive(Debug, Clone)]
struct Planet {
    name: String,
    class: String,
    stage: String,
    radius: f32,
    orbital: bool,
    loc: Location,
}

#[derive(Debug, Clone)]
enum Faction {
    Rebels,
    Imperials
}

#[derive(Debug, Clone)]
enum Weapon {
    Melee,
    Ranged
}

#[derive(Debug, Clone)]
enum Ships {
    XWing(String, Faction),
    YWing(String, Faction),
    TieFighter(String, Faction),
    TieBomber(String, Faction),
}

#[derive(Debug, Clone)]
struct Character {
    name: String,
    home_planet: Planet,
    weapon: Weapon,
    dmg: f32,
}

#[derive(Debug, Clone)] //, Default)] ??
pub struct Ship {
    pilot: Option<Character>, // call sign?
    kind: Option<Ships>,
    kills: u32,
    fuel: f32,
    health: f32, // Default::default(), ??
    in_flight: bool,
    grounded: bool,
    rebel_scum: bool,
    dmg: f32,
}

trait New {
    fn new(&self) -> Ship;
}

impl New for Ship {
    fn new(&self) -> Ship {
         Ship {
             pilot: None,
             health: 100.0,
             dmg: 100.0,
             fuel: 100.0,
             grounded: false,
             in_flight: false,
             kills: 0,
             kind: None,
             rebel_scum: false
         }
     }
 }

//pub trait Xed {}

//impl Xed for Ship {}

//impl Xed for Character {}

pub trait Shoot {
    fn shoot(&self, other: Ship) -> ();
}

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

 impl Shoot for Ship {
     fn shoot(&self, other: Ship) {
         if let Some(c) = other.pilot && let Some(x) = other.kind {
             println!("\n{:?} ({:?}) shot at {:?} ({:?})!",
                      self.pilot,
                      self.kind,
                      c.name,
                      other.kind
             );
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

//noinspection ALL
fn can_travel(ship: &Ship) {
    let distance: f32 = match ship.kind {
        Some(Ships::TieBomber(..)) | Some(Ships::TieFighter(..)) => 1.0 * ship.fuel,
        Some(Ships::XWing(..)) | Some(Ships::YWing(..)) => 10.0 * ship.fuel,
        None => 0.0,
    };
    println!("{:?} ({:?}) can travel a distance of {:?} lightyears.",
             ship.pilot, ship.kind, distance);
}

fn main() {

    let mut rng = rand::thread_rng();

    let tatooine = Planet {
        name: String::from("Tatooine"),
        class: String::from("M"),
        stage: String::from("Spez"),
        radius: 10.5,
        orbital: false,
        loc: Location { x: 7.2, y: 3.4, z: 5.6 },
    };

    let luke = Character {
        name: String::from("Luke"),
        home_planet: tatooine.clone(),
        weapon: Weapon::Melee,
        dmg: 10.0,
    };
    let porkins = Character {
        name: String::from("Porkins"),
        home_planet: tatooine.clone(),
        weapon: Weapon::Melee,
        dmg: 3.5,
    };
    let vader = Character {
        name: String::from("Darth Vader"),
        home_planet: tatooine.clone(),
        weapon: Weapon::Melee,
        dmg: 11.5,
    };

    println!("{:?}\n", tatooine);

    let red_five = Ship {
        pilot: Some(luke),
        kind: Some(Ships::XWing("AA-589".to_string(), Faction::Rebels)),
        kills: 18,
        fuel: 0.13,
        health: 0.9,
        in_flight: true,
        grounded: false,
        rebel_scum: true,
        dmg: rng.gen(),
    };
    println!("{:?}", &red_five);
    can_travel(&red_five);

    let red_six = Ship {
        pilot: Some(porkins),
        kind: Some(Ships::XWing("AA-399".to_string(), Faction::Rebels)),
        kills: 4,
        fuel: 0.56,
        ..red_five
    };
    println!("{:?}", &red_six);
    can_travel(&red_six);

    let tie_one = Ship {
        pilot: Some(vader),
        kind: Some(Ships::TieBomber("TB-9".to_string(), Faction::Imperials)),
        kills: 493,
        fuel: 0.9,
        health: 1.0,
        in_flight: true,
        grounded: false,
        rebel_scum: false,
        dmg: rng.gen(),
    };
    println!("{:?}", &tie_one);
    can_travel(&tie_one);

    tie_one.shoot(red_five.clone());
    &red_five.shoot(tie_one.clone());
    red_five.shoot(tie_one);

    // rng & turbofish testing
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("\nRandom u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
    println!("Random float in range: {}", rng.gen_range(0.0, 10.0));
}

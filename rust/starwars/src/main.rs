use rand::prelude::*;
use cgmath::prelude::*;

#[derive(Debug)]
struct Location {
    x: f32, 
    y: f32, 
    z: f32,
}

#[derive(Debug)]
struct Planet {
    name: String,
    class: String,
    stage: String,
    radius: f64,
    orbital: bool,
    loc: Location,
}

#[derive(Debug)]
enum Faction {
    Rebels,
    Imperials
}

#[derive(Debug)]
enum Weapon {
    Melee,
    Ranged
}

#[derive(Debug)]
enum Ships {
    XWing(String, Faction),
    YWing(String, Faction),
    TieFighter(String, Faction),
    TieBomber(String, Faction),
}

#[derive(Debug)]
struct Character {
    name: String,
    home_planet: Planet,
    weapon: Weapon
}

#[derive(Debug)] //, Default)] ??
pub struct Ship {
    pilot: Character, // call sign?
    kind: Ships,
    kills: u32,
    fuel: f32,
    health: u8, // Default::default(), ??
    in_flight: bool,
    grounded: bool,
    rebel_scum: bool,
    dmg: f64,
}
// impl Default for Ship {
//     fn default(&self) -> Ship {
//         Ship { health: 100 }
//     }
// } ??

pub trait Xed {}

impl Xed for Ship {}

impl Xed for Character {}

pub trait Shoot {
    fn shoot<T>(&self, other: &T) -> String;
}

impl<T> Shoot for T
    where
        T: Xed,
{
    fn shoot<T>(&self, other: &T) {
        println!("\n{:?} ({:?}) shot at {:?} ({:?})!",
                 self.pilot,
                 self.kind,
                 other.pilot,
                 other.kind
        );
    }
}

// impl Shoot for Ship {
//     fn shoot<T>(&self, other: &T) {
//         println!("\n{:?} ({:?}) shot at {:?} ({:?})!",
//                  self.pilot,
//                  self.kind,
//                  other.pilot,
//                  other.kind
//         );
//     }
// }

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
        Ships::TieBomber(..) | Ships::TieFighter(..) => 1.0 * ship.fuel,
        Ships::XWing(..) | Ships::YWing(..) => 10.0 * ship.fuel,
    };
    println!("{} ({:?}) can travel a distance of {} lightyears.", ship.pilot, ship.kind, distance);
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
        home_planet: tatooine.copy(),
        dmg: 10.0,
    };
    let porkins = Character {
        name: String::from("Porkins"),
        home_planet: tatooine.copy(),
        dmg: 3.5,
    };
    let vader = Character {
        name: String::from("Darth Vader"),
        home_planet: tatooine.copy(),
        dmg: 11.5,
    };

    println!("{:?}\n", tatooine);

    let red_five = Ship {
        pilot: luke,
        kind: Ships::XWing("AA-589".to_string(), Faction::Rebels),
        kills: 18,
        fuel: 0.13,
        health: 90,
        in_flight: true,
        grounded: false,
        rebel_scum: true,
        dmg: rng.gen(),
    };
    println!("{:?}", &red_five);
    can_travel(&red_five);

    let red_six = Ship {
        pilot: porkins,
        kind: Ships::XWing("AA-399".to_string(), Faction::Rebels),
        kills: 4,
        fuel: 0.56,
        ..red_five
    };
    println!("{:?}", &red_six);
    can_travel(&red_six);

    let tie_one = Ship {
        pilot: vader,
        kind: Ships::TieBomber("TB-9".to_string(), Faction::Imperials),
        kills: 493,
        fuel: 0.9,
        health: 100,
        in_flight: true,
        grounded: false,
        rebel_scum: false,
        dmg: rng.gen(),
    };
    println!("{:?}", &tie_one);
    can_travel(&tie_one);

    tie_one.shoot(&red_five);
    red_five.shoot(&tie_one);
    red_five.shoot(&tie_one);

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

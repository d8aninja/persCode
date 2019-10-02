//use cgmath::prelude::*;
use rand::prelude::*;
use s2::r3::vector::Vector as r3_vec;

mod troops;
use troops::{
    Character,
    Faction,
    Weapon,
};
mod ships;
use ships::{
    Ship,
    Ships,
    Shoot,
    can_travel,
};
mod space;
use space::{
    Star,
    Planet,
    System,
};

fn main() {

    let mut rng = rand::thread_rng();

    let tatooine = Planet {
        name: String::from("Tatooine"),
        class: String::from("M"),
        stage: String::from("Spez"),
        radius: 10.5,
        orbital: false,
        loc: r3_vec{ x: 7.2, y: 3.4, z: 5.6 },
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
        loc: r3_vec{x: 3.23, y: 4.61, z: 5.94},
    };
//    println!("{:?}", &red_five);
    can_travel(&red_five);

    let red_six = Ship {
        pilot: Some(porkins),
        kind: Some(Ships::XWing("AA-399".to_string(), Faction::Rebels)),
        kills: 4,
        fuel: 0.56,
        loc: r3_vec{x: 3.13, y: 4.62, z: 5.95},
        ..red_five
    };
//    println!("{:?}", &red_six);
    can_travel(&red_six);

    let tie_one = Ship {
        pilot: Some(vader),
        kind: Some(Ships::TIEBomber("TB-9".to_string(), Faction::Imperials)),
        kills: 493,
        fuel: 0.9,
        health: 1.0,
        in_flight: true,
        grounded: false,
        rebel_scum: false,
        dmg: rng.gen(),
        loc: r3_vec{x: 3.15, y: 4.55, z: 5.92}
    };
//    println!("{:?}", &tie_one);
    can_travel(&tie_one);

    tie_one.shoot(red_five.clone());
    &red_five.shoot(tie_one.clone());
    red_five.shoot(tie_one.clone());

    println!("\n\n Distance from TIE One to RED Six: {}", &tie_one.loc.distance(&red_six.loc));

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

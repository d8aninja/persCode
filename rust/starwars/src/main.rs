use rand::prelude::*;

struct Planet {
    name: String,
    class: String,
    stage: u8,
    radius: u64,
    orbital: bool,
}

#[derive(Debug)]
enum Faction {
    Rebels,
    Imperials
}

#[derive(Debug)]
enum Ships {
    XWing(String, Faction),
    YWing(String, Faction),
    TieFighter(String, Faction),
    TieBomber(String, Faction),
}

#[derive(Debug)] //, Default)] ??
struct Ship {
    pilot: String, // call sign?
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
//     fn default() -> Ship {
//         Ship { health: 100 }
//     }
// } ??

impl Ship {
    fn shoot(&self, other: &Ship) {
        println!("\n{:?} ({:?}) shot at {:?} ({:?})!", 
            self.pilot, 
            self.kind, 
            other.pilot, 
            other.kind
        );
    }
}

fn main() {

    let mut rng = rand::thread_rng();

    let red_five = Ship {
        pilot: String::from("Luke"),
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
        pilot: String::from("Porkins"),
        kind: Ships::XWing("AA-399".to_string(), Faction::Rebels),
        kills: 4,
        fuel: 0.56,
        ..red_five
    };
    println!("{:?}", &red_six);
    can_travel(&red_six);

    let tie_one = Ship {
        pilot: String::from("Vader".to_string()),
        kind: Ships::TieBomber("TB-9".to_string(), Faction::Imperials),
        kills: 493,
        fuel: 0.9,
        health: 100,
        in_flight: true,
        grounded: false,
        rebel_scum: false,
        dmg: rng.gen(),
    };

    tie_one.shoot(&red_five);
    red_five.shoot(&tie_one);
    red_five.shoot(&tie_one);
}

fn can_travel(ship: &Ship) {
    // let distance = ship.fuel * 10.0;
    let distance = match ship.kind {
        Ships::TieBomber | Ships::TieFighter => ship.fuel * 1.0,
        Ships::XWing | Ships::YWing => ship.fuel * 10.0,
    };
    println!("{} ({:?}) can travel a distance of {} lightyears.", ship.pilot, ship.kind, distance);
}

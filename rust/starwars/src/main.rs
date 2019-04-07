#[warn(dead_code)]
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

#[derive(Debug)]
struct Ship {
    pilot: String, // call sign?
    kind: Ships,
    kills: u32,
    fuel: f32,
    health: Default::default(),
    in_flight: bool,
    grounded: bool,
    rebel_scum: bool,
}

fn main() {
    let red_five = Ship {
        pilot: String::from("Luke"),
        kind: Ships::XWing("AA-589".to_string(), Faction::Rebels),
        kills: 18,
        fuel: 0.13,
        in_flight: true,
        grounded: false,
        rebel_scum: true,
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
}

fn can_travel(ship: &Ship) {
    let distance = ship.fuel * 10.0;
    println!("{} ({:?}) can travel a distance of {} lightyears.", ship.pilot, ship.kind, distance);
}

//use cgmath::prelude::*;
mod troops;

use troops::*;

mod ships;

use ships::*;

mod space;

use space::*;

fn main() {
    /*
    let mut rng = rand::thread_rng();
    rng & turbofish testing
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("\nRandom u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
    println!("Random float in range: {}", rng.gen_range(0.0, 10.0));
    */
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use s2::r3::vector::Vector as r3_vec;
    use super::ships::*;
    use super::space::*;
    use super::troops::*;

    #[test]
    fn test_troops() {
        let alderaan = Planet::default();
        let leia = Character {
            name: String::from("Leia"),
            home_planet: Some(alderaan),
            weapon: Some(Weapon::Ranged),
            dmg: Some(10.0),
        };
        assert_eq!(leia.name, "Leia".to_string())
    }

    #[test]
    //#[ignore]
    fn test_ships() {
        //deps on rng :(
        let mut rng = rand::thread_rng();
        let luke = Character::default();
        let vader = Character::default();

        let red_five = Ship {
            pilot: Some(luke),
            // Faction reaches across to troops mod...ok?
            kind: Some(Ships::XWing("AA-589".to_string(), Faction::Rebels)),
            kills: 18,
            fuel: 13,
            health: 9,
            in_flight: 1,
            grounded: 0,
            rebel_scum: 1,
            dmg: rng.gen::<f32>(),
            loc: r3_vec { x: 3.23, y: 4.61, z: 5.94 },
        };

        let tie_one = Ship {
            pilot: Some(vader),
            kind: Some(Ships::TIEBomber("TB-9".to_string(), Faction::Imperials)),
            kills: 493,
            fuel: 90,
            health: 100,
            in_flight: 1,
            grounded: 0,
            rebel_scum: 0,
            dmg: rng.gen::<f32>(),
            loc: r3_vec { x: 3.15, y: 4.55, z: 5.92 },
        };

        let distance = travel_distance(&red_five);
        assert_eq!(
            distance,
            130
//            format!("{:?} ({:?}), from {:?} can travel a distance of {:?} light-years.",
//                    &red_five.pilot.as_ref().unwrap().name,
//                    &red_five.kind.as_ref().unwrap(),
//                    &red_five.pilot.as_ref().unwrap().home_planet.name,
//                    distance
//            )
        );

        //todo: test shooting
        //tie_one.shoot(red_five.clone());
        //&red_five.shoot(tie_one.clone());
        //red_five.shoot(tie_one.clone());

        //todo: assertion; parameterize the names, like above
        //println!("\n\n Distance from TIE One to RED Six: {}", &tie_one.loc.distance(&red_six.loc));
    }

    #[test]
    fn test_space() {
        let tatooine = Planet {
            name: String::from("Tatooine"),
            class: Some(String::from("M")),
            stage: Some(String::from("Spez")),
            radius: Some(10.5),
            orbital: Some(false),
            loc: r3_vec { x: 7.2, y: 3.4, z: 5.6 },
        };
        assert_eq!(tatooine.name, "Tatooine".to_string())
    }
}


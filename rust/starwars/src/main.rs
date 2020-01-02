//use cgmath::prelude::*;
#[allow(unused_imports)]
mod troops;

#[allow(unused_imports)]
use troops::*;

#[allow(unused_imports)]
mod ships;

#[allow(unused_imports)]
use ships::*;

#[allow(unused_imports)]
mod space;

#[allow(unused_imports)]
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
    use super::ships::*;
    use super::space::*;
    use super::troops::*;
    use rand::prelude::*;
    use s2::r3::vector::Vector as r3_vec;
    use std::borrow::Borrow;
    use std::convert::TryInto;

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
    fn test_ships() {
        //deps on rng :(
        let mut rng = rand::thread_rng();
        let luke = Character::new(
            "Luke".to_string(),
            Some(Planet::new(
                "Tatooine".to_string(),
                None,
                None,
                None,
                None,
                r3_vec {
                    x: 1.7,
                    y: 3.1,
                    z: 2.4,
                }, //this needs to be boxed
            )),
            Some(Weapon::Melee),
            Some(34.2),
        );
        let vader = Character::new(
            "Anakin".to_string(),
            Some(Planet::new(
                "Tatooine".to_string(),
                None,
                None,
                None,
                None,
                r3_vec {
                    x: 1.7,
                    y: 3.1,
                    z: 2.4,
                }, //this needs to be boxed
            )),
            Some(Weapon::Melee),
            Some(55.0),
        );
        //todo: needs test
        let celebrity_cameo = Character::default();

        let red_five = Ship::new(
            Some(luke),
            // Faction reaches across to troops mod...ok?
            Some(Ships::XWing("AA-589".to_string(), Faction::Rebels)),
            18,
            13,
            9,
            1,
            0,
            1,
            rng.gen::<f32>(),
            r3_vec {
                x: 3.23,
                y: 4.61,
                z: 5.94,
            },
        );

        let tie_one = Ship::new(
            Some(vader),
            Some(Ships::TIEBomber("TB-9".to_string(), Faction::Imperials)),
            493,
            90,
            100,
            1,
            0,
            0,
            rng.gen::<f32>(),
            r3_vec {
                x: 3.15,
                y: 4.55,
                z: 5.92,
            },
        );

        let distance = travel_distance(&red_five);
        assert_eq!(distance, 130);

        //            format!("{:?} ({:?}), from {:?} can travel a distance of {:?} light-years.",
        //                    &red_five.pilot.as_ref().unwrap().name,
        //                    &red_five.kind.as_ref().unwrap(),
        //                    &red_five.pilot.as_ref().unwrap().home_planet.name,
        //                    distance
        //            )

        //todo: make assertion
        println!(
            "{}",
            format!(
                "{:?} ({:?}) shot at {:?} ({:?})!",
                red_five.pilot.unwrap().name.bytes(),
                red_five.kind.unwrap(),
                tie_one.pilot.unwrap().name,
                tie_one.kind.unwrap().clone(),
            )
        );

        //todo: test shooting
        //red_five.shoot(tie_one),
        println!("{}",
            format!("{:?} ({:?}) shot at {:?} ({:?})!",
                    red_five.pilot.unwrap().name,
                    red_five.kind.unwrap().to_owned(),
                    tie_one.pilot.unwrap().name,
                    tie_one.kind.unwrap().to_owned()
            )
        );

        //todo: assertion; parameterize the names
        //println!("\n\n Distance from TIE One to RED Six: {}", &tie_one.loc.distance(&red_six.loc));
    }

    #[test]
    fn test_space() {
        let tatooine = Planet::new(
            String::from("Tatooine"),
            Some(String::from("M")),
            Some(String::from("Spez")),
            Some(10.5),
            Some(true), //3 moons
            r3_vec {
                x: 7.2,
                y: 3.4,
                z: 5.6,
            },
        );
        assert_eq!(tatooine.name, "Tatooine".to_string());

        let ohann = Planet::new(
            String::from("Ohan"),
            Some(String::from("C")),
            Some(String::from("Spez")),
            Some(8.7),
            Some(true), //3 moons
            r3_vec {
                x: 7.25,
                y: 3.28,
                z: 5.77,
            },
        );
        let adrianna = Planet::new(
            String::from("Adrianna"),
            Some(String::from("M")),
            Some(String::from("Spez")),
            Some(12.3),
            Some(true), //4 moons
            r3_vec {
                x: 7.18,
                y: 3.5,
                z: 5.4,
            },
        );

        //todo: figure out how to make the centroid with points from the planet's r3_vecs
//        let a = s2::r2::point::Point::from_coords(x: tatooine.loc.x, y: tatooine.loc.y, z: tatooine.loc.z);
//        let b = s2::r2::point::Point::from_coords(x: ohann.loc.x, y: ohann.loc.y, z: ohann.loc.z);
//        let c = s2::r2::point::Point::from_coords(x: adrianna.loc.x, y: adrianna.loc.y, z: adrianna.loc.z);
//        let centroid =  s2::point::planar_centroid(a, b, c);
//        dbg!(tatoo);
//        dbg!(centroid);

        let tatoo = System::new(
            "Tatoo".to_string(),
            None,
            None,
            Some(7.2),
            Some(false),
            r3_vec{ x: 7.23, y: 3.46, z: 5.58 }, //use centroid here somehow
            Some(vec![tatooine, ohann, adrianna]),
            None,
        );

        dbg!(tatoo);
    }
}

//pub(crate) mod space;
use s2::r3::vector::Vector as r3_vec;

#[derive(Debug, Clone)]
pub struct Star;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Planet {
    pub name: String,
    pub class: Option<String>,
    pub stage: Option<String>,
    pub radius: Option<f32>,   // Kkm
    pub orbital: Option<bool>, //needs to reflect count, vec?
    pub loc: r3_vec,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct System {
    pub name: String,
    pub class: Option<String>,
    pub stage: Option<String>,
    pub radius: Option<f32>, // light-years
    pub orbital: Option<bool>,
    pub loc: r3_vec,
    pub planets: Option<Vec<Planet>>,
    pub stars: Option<Vec<Star>>,
}

impl Default for Planet {
    fn default() -> Self {
        Planet {
            name: "Unknown".to_string(),
            class: None,
            stage: None,
            radius: None, // light-years
            orbital: None,
            loc: r3_vec {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

impl Planet {
    pub fn new(
        name: String,
        class: Option<String>,
        stage: Option<String>,
        radius: Option<f32>,
        orbital: Option<bool>,
        loc: r3_vec,
    ) -> Self {
        Planet {
            name,
            class,
            stage,
            radius, // light-years
            orbital,
            loc,
        }
    }
}

impl System {
    pub fn new(
        name: String,
        class: Option<String>,
        stage: Option<String>,
        radius: Option<f32>, // light-years
        orbital: Option<bool>,
        loc: r3_vec,
        planets: Option<Vec<Planet>>,
        stars: Option<Vec<Star>>,
    ) -> Self {
        System {
            name,
            class,
            stage,
            radius,
            orbital,
            loc,
            planets,
            stars,
        }
    }
}

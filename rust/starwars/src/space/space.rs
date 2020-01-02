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

use s2::r3::vector::Vector as r3_vec;

use crate::troops::{Character, Faction};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Ship {
    pub pilot: Option<Character>,
    pub kind: Option<Ships>,
    pub kills: i32,     // can be -1 for unk
    pub fuel: i32,      // can be -1 for unk
    pub health: i32,    // can be -1 for unk
    pub in_flight: i8,  // -1,0,1
    pub grounded: i8,   // 1,0,1; should react to health
    pub rebel_scum: i8, // -1,0,1
    pub dmg: f32,       // can be -1.0 for unk
    pub loc: r3_vec,    // todo: consider {-0.01,-0.01,-0.01} for unk loc
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Ships {
    XWing(String, Faction),
    YWing(String, Faction),
    TIEFighter(String, Faction),
    TIEBomber(String, Faction),
}

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Qualification {
    Starter,
    Senior,
    Top,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Position {
    Melee,
    Ranged,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Class {
    Guard,
    Medic,
    Vanguard,
    Caster,
    Sniper,
    Defender,
    Supporter,
    Specialist,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Affix {
    Healing,
    Support,
    DPS,
    AoE,
    Slow,
    Survival,
    Defense,
    Debuff,
    Shift,
    CrowdControl,
    Nuker,
    Summon,
    FastRedeploy,
    DPRecovery,
    Robot,
}

#[derive(Deserialize, Debug)]
pub struct Operator {
    name: String,
    rarity: u8,
    qualification: Option<Qualification>,
    position: Position,
    class: Class,
    affix: Vec<Affix>,
}

impl Operator {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_rarity(&self) -> u8 {
        self.rarity
    }

    pub fn get_qualification(&self) -> Option<Qualification> {
        self.qualification.clone()
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn get_class(&self) -> Class {
        self.class.clone()
    }

    pub fn get_affix(&self) -> Vec<Affix> {
        self.affix.clone()
    }
}
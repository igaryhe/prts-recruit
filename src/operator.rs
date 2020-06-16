use serde::{ Serialize, Deserialize };
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Tag {
    #[serde(alias = "新手")]
    Starter,
    #[serde(alias = "资深干员")]
    Senior,
    #[serde(alias = "高级资深干员")]
    Top,
    #[serde(alias = "近战位")]
    Melee,
    #[serde(alias = "远程位")]
    Ranged,
    #[serde(alias = "重装干员")]
    Defender,
    #[serde(alias = "近卫干员")]
    Guard,
    #[serde(alias = "先锋干员")]
    Vanguard,
    #[serde(alias = "狙击干员")]
    Sniper,
    #[serde(alias = "术师干员")]
    Caster,
    #[serde(alias = "医疗干员")]
    Medic,
    #[serde(alias = "辅助干员")]
    Supporter,
    #[serde(alias = "特种干员")]
    Specialist,
    #[serde(alias = "治疗")]
    Healing,
    #[serde(alias = "支援")]
    Support,
    #[serde(alias = "输出")]
    DPS,
    #[serde(alias = "群攻")]
    AoE,
    #[serde(alias = "减速")]
    Slow,
    #[serde(alias = "生存")]
    Survival,
    #[serde(alias = "防护")]
    Defense,
    #[serde(alias = "削弱")]
    Debuff,
    #[serde(alias = "位移")]
    Shift,
    #[serde(alias = "控场")]
    CrowdControl,
    #[serde(alias = "爆发")]
    Nuker,
    #[serde(alias = "召唤")]
    Summon,
    #[serde(alias = "快速复活")]
    FastRedeploy,
    #[serde(alias = "费用回复")]
    DPRecovery,
    #[serde(alias = "支援机械")]
    Robot,
}

#[derive(Deserialize, Debug)]
pub struct Operator {
    name: String,
    rarity: u8,
    qualification: Option<Tag>,
    position: Tag,
    class: Tag,
    affix: Vec<Tag>,
}

impl Operator {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_rarity(&self) -> u8 {
        self.rarity
    }

    pub fn get_qualification(&self) -> Option<Tag> {
        self.qualification.clone()
    }

    pub fn get_position(&self) -> Tag {
        self.position.clone()
    }

    pub fn get_class(&self) -> Tag {
        self.class.clone()
    }

    pub fn get_affix(&self) -> Vec<Tag> {
        self.affix.clone()
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.name, self.rarity)
    }
}

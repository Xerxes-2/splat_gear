#[derive(Clone, Copy)]
pub enum Brand {
    Amiibo,
    Annaki,
    Barazushi,
    Cuttlegear,
    Emberz,
    Enperry,
    Firefin,
    Forge,
    Grizzco,
    Inkline,
    KrakOn,
    Rockenberg,
    Skalop,
    Splash,
    SquidForce,
    Takoroka,
    Tentatek,
    ToniKensa,
    Zekko,
    Zink,
}

use super::ability::Ability;
const PREFER: [Option<(Ability, Ability)>; 20] = [
    None,
    Some((Ability::SubSave, Ability::SpecialSave)),
    Some((Ability::Intensify, Ability::SubPower)),
    None,
    Some((Ability::Intensify, Ability::SpecialCharge)),
    Some((Ability::SubPower, Ability::InkResistance)),
    Some((Ability::SubSave, Ability::InkRecovery)),
    Some((Ability::SpecialPower, Ability::SubSave)),
    None,
    Some((Ability::SubResistance, Ability::Intensify)),
    Some((Ability::SwimSpeed, Ability::SubResistance)),
    Some((Ability::RunSpeed, Ability::SwimSpeed)),
    Some((Ability::QuickRespawn, Ability::SpecialSave)),
    Some((Ability::MainSave, Ability::RunSpeed)),
    Some((Ability::InkResistance, Ability::MainSave)),
    Some((Ability::SpecialCharge, Ability::SpecialPower)),
    Some((Ability::InkRecovery, Ability::SuperJump)),
    Some((Ability::MainSave, Ability::SubPower)),
    Some((Ability::SpecialSave, Ability::SpecialCharge)),
    Some((Ability::SuperJump, Ability::QuickRespawn)),
];

use std::mem::transmute;
impl From<u8> for Brand {
    fn from(x: u8) -> Self {
        unsafe { transmute(x % 20) }
    }
}

impl Brand {
    pub fn max_roll(&self, drink: Option<Ability>) -> u32 {
        match drink {
            Some(drink) => {
                let prefer = PREFER[drink as usize];
                match prefer {
                    Some((common, uncommon)) => {
                        if drink == common {
                            25
                        } else if drink == uncommon {
                            34
                        } else {
                            33
                        }
                    }
                    None => 26,
                }
            }
            None => match *self {
                Self::Amiibo => 28,
                Self::Cuttlegear => 28,
                Self::Grizzco => 28,
                _ => 35,
            },
        }
    }

    pub fn weight(&self, a: Ability) -> u32 {
        let prefer = PREFER[*self as usize];
        match prefer {
            Some((common, uncommon)) => {
                if a == common {
                    10
                } else if a == uncommon {
                    1
                } else {
                    2
                }
            }
            None => 2,
        }
    }
}

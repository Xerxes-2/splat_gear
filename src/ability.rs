#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Ability {
    MainSave,
    SubSave,
    InkRecovery,
    RunSpeed,
    SwimSpeed,
    SpecialCharge,
    SpecialSave,
    SpecialPower,
    QuickRespawn,
    SuperJump,
    SubPower,
    InkResistance,
    SubResistance,
    Intensify,
}

use std::mem::transmute;
impl From<u32> for Ability {
    fn from(x: u32) -> Self {
        unsafe { transmute((x % 14) as u8) }
    }
}

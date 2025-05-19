use screeps::Room;

const _TIER_1: u32 = 300;
const TIER_2: u32 = 550;
const TIER_3: u32 = 800;
const TIER_4: u32 = 1300;
const TIER_5: u32 = 1800;
const TIER_6: u32 = 2300;
const TIER_7: u32 = 5600;
const _TIER_8: u32 = 10000;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Tier {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Tier {
    pub fn get_for_room(room: &Room) -> Self {
        let energy_available = room.energy_capacity_available();
        let controller_level = room.controller().unwrap().level();
        match energy_available {
            e if e >= TIER_7 && controller_level == 8 => Tier::Eight,
            e if e >= TIER_7 => Tier::Seven,
            e if e >= TIER_6 => Tier::Six,
            e if e >= TIER_5 => Tier::Five,
            e if e >= TIER_4 => Tier::Four,
            e if e >= TIER_3 => Tier::Three,
            e if e >= TIER_2 => Tier::Two,
            _ => Tier::One, // Default case
        }
    }
}

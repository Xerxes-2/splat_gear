pub mod ability;
pub mod brand;
pub mod solution;

pub const PREDICT: usize = 5;
pub const ABILITY_SIZE: usize = 14;

pub fn advance_seed(mut x: u32) -> u32 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^ x << 5
}

pub fn get_brand_ability(
    seed: u32,
    brand: brand::Brand,
    drink: Option<ability::Ability>,
) -> ability::Ability {
    let roll = seed % brand.max_roll(drink);
    let mut sum: u32 = 0;
    for i in 0..20 {
        if drink.is_some() && drink.unwrap() == ability::Ability::from(i) {
            continue;
        }
        sum += brand.weight(ability::Ability::from(i));
        if roll < sum {
            return ability::Ability::from(i);
        }
    }
    unreachable!()
}

pub fn get_ability(
    mut seed: u32,
    brand: brand::Brand,
    drink: Option<ability::Ability>,
) -> (u32, ability::Ability) {
    seed = advance_seed(seed);
    match drink {
        Some(drink) => {
            if seed % 100 <= 29 {
                return (seed, drink);
            }
            seed = advance_seed(seed);
            (seed, get_brand_ability(seed, brand, Some(drink)))
        }
        None => (seed, get_brand_ability(seed, brand, None)),
    }
}

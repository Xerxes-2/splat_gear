pub mod ability;
pub mod brand;
pub mod solution;

pub const PREDICT: usize = 6;
pub const ABILITY_SIZE: usize = 14;

/// Advance the RNG by one step.
///
/// # Arguments
///
/// * `seed` - The seed to advance.
///
/// # Returns
///
/// The new seed.
///
/// # Examples
///
/// ```
/// use splat_gear::advance_seed;
///     assert_eq!(advance_seed(4256839584), 0x2F6A3C59);
/// ```
pub fn advance_seed(mut x: u32) -> u32 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^ x << 5
}

/// Get the brand ability without drink effect.
///
/// # Arguments
///
/// * `seed` - The seed.
/// * `brand` - The brand.
/// * `drink` - The drink.
///
/// # Returns
///
/// The brand ability.
///
/// # Examples
///
/// ```
/// use splat_gear::ability::Ability;
/// use splat_gear::brand::Brand;
/// use splat_gear::get_brand_ability;
///    assert_eq!(get_brand_ability(0xd2832ae7, Brand::Amiibo, Some(Ability::SpecialPower)), Ability::SubSave);
pub fn get_brand_ability(
    seed: u32,
    brand: brand::Brand,
    drink: Option<ability::Ability>,
) -> ability::Ability {
    let roll = seed % brand.max_roll(drink);
    let mut sum: u32 = 0;
    for i in 0..ABILITY_SIZE {
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

/// Get the ability for a given seed with drink or not.
///
/// # Arguments
///
/// * `seed` - The current seed of gear.
/// * 'brand' - The brand of gear.
/// * `drink` - The drink, can be none.
///
/// # Returns
///
/// New seed, ability.
///
/// # Examples
///
/// ```
/// use splat_gear::{get_ability, brand::Brand, ability::Ability};
///    assert_eq!(get_ability(0xA7B92259, Brand::Amiibo, Some(Ability::SpecialPower)), (0xd2832ae7, Ability::SubSave));
/// ```
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

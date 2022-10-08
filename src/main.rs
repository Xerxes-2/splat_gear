use splat_gear::ability::Ability;
use splat_gear::brand::Brand;
use splat_gear::get_ability;

fn drink_during(
    mut seed: u32,
    brand: Brand,
    drink: Option<Ability>,
    begin: usize,
    end: usize,
) -> [Ability; 5] {
    let mut ret = [Ability::MainSave; 5];
    for i in 0..5 {
        (seed, ret[i]) = if i >= begin && i < end {
            get_ability(seed, brand, drink)
        } else {
            get_ability(seed, brand, None)
        };
    }
    ret
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Quality {
    ABA,
    AA,
    AAA,
}
struct Solution {
    drink: Option<Ability>,
    begin: usize,
    end: usize,
    qual: Quality,
    appear: usize,
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.begin != self.end {
            write!(
                f,
                "Drink {:?} during {}-{}, {:?} appears at {}",
                self.drink.unwrap(),
                self.begin,
                self.end - 1,
                self.qual,
                self.appear
            )
        } else {
            write!(
                f,
                "Drink nothing during, {:?} appears at {}",
                self.qual, self.appear
            )
        }
    }
}

fn search_solution(seed: u32, brand: Brand, target: Ability) -> Vec<Solution> {
    let mut ret = Vec::new();
    for drink in 0..20 {
        let drink = Ability::from(drink);
        for begin in 0..5 {
            for end in begin..6 {
                if begin != 0 && end == begin {
                    continue;
                }
                let abilities = drink_during(seed, brand, Some(drink), begin, end);
                let mut best: Option<Solution> = None;
                for i in 0..3 {
                    let count = &abilities[i..i + 2]
                        .into_iter()
                        .filter(|&&a| a == target)
                        .count();
                    if count == &3 {
                        best = Some(Solution {
                            drink: Some(drink),
                            begin,
                            end,
                            qual: Quality::AAA,
                            appear: i,
                        });
                        break;
                    } else if count == &2 {
                        let qual = if abilities[i + 1] == target {
                            Quality::ABA
                        } else {
                            Quality::AA
                        };
                        match best {
                            Some(ref mut best) => {
                                if best.qual < qual {
                                    best.qual = qual;
                                }
                            }
                            None => {
                                best = Some(Solution {
                                    drink: Some(drink),
                                    begin,
                                    end,
                                    qual,
                                    appear: i,
                                });
                            }
                        }
                    }
                }
                if best.is_some() {
                    ret.push(best.unwrap());
                }
            }
        }
    }
    ret
}

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter seed: (0x?)");
        std::io::stdin().read_line(&mut input).unwrap();
        let seed = u32::from_str_radix(&input.trim_start_matches("0x").trim(), 16).unwrap();
        println!(
            "Enter brand by number: 
    0: Amiibo,
    1: Annaki 无法无天,
    2: Barazushi 散寿司,
    3: Cuttlegear 鱼干制造,
    4: Emberz 七轮,
    5: Enperry 鱿皇,
    6: Firefin 暖流,
    7: Forge 锻品,
    8: Grizzco 熊先生商会,
    9: Inkline 时雨,
    10: KrakOn 海月,
    11: Rockenberg 罗肯贝格,
    12: Skalop 帆立,
    13: Splash 寺门,
    14: SquidForce 战斗鱿鱼,
    15: Takoroka 暇古,
    16: Tentatek 艾洛眼,
    17: ToniKensa 剑尖鱿,
    18: Zekko 泽酷,
    19: Zink 钢铁先锋"
        );
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let brand = Brand::from(input.trim().parse::<u8>().unwrap());
        println!(
            "Enter target ability:
    0: MainSave,
    1: SubSave,
    2: InkRecovery,
    3: RunSpeed,
    4: SwimSpeed,
    5: SpecialCharge,
    6: SpecialSave,
    7: SpecialPower,
    8: QuickRespawn,
    9: SuperJump,
    10: SubPower,
    11: InkResistance,
    12: SubResistance,
    13: Intensify"
        );
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let target = Ability::from(input.trim().parse::<u32>().unwrap());
        let sols = search_solution(seed, brand, target);
        if sols.len() == 0 {
            println!("No solution found");
        } else {
            for sol in sols {
                println!("{}", sol);
            }
        }
    }
}

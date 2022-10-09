use itertools::iproduct;
use splat_gear::ability::Ability;
use splat_gear::brand::Brand;
use splat_gear::get_ability;
use splat_gear::solution::Quality;
use splat_gear::solution::Solution;
use splat_gear::ABILITY_SIZE;
use splat_gear::PREDICT;

const MAX_DISPLAY: usize = 20;

fn search_solution(
    original_seed: u32,
    brand: Brand,
    target: Ability,
    standard: Quality,
) -> Vec<Solution> {
    let mut ret = Vec::new();
    let range = (0..ABILITY_SIZE + 1).map(|i| {
        if i == 0 {
            None
        } else {
            Some(Ability::from(i - 1))
        }
    });
    for i in iproduct!(
        range.clone(),
        range.clone(),
        range.clone(),
        range.clone(),
        range.clone()
    ) {
        let mut abilities: [Ability; PREDICT] = [Ability::MainSave; PREDICT];
        let drinks: [Option<Ability>; PREDICT] = [i.0, i.1, i.2, i.3, i.4];
        let mut seed = original_seed;
        for j in 0..PREDICT {
            (seed, abilities[j]) = get_ability(seed, brand, drinks[j]);
        }
        let sol = Solution::from((drinks, abilities, target));
        if sol.qual >= standard {
            ret.push(sol);
        }
    }
    ret.sort_by(|b, a| {
        a.qual
            .cmp(&b.qual)
            .then(b.cost.cmp(&a.cost))
            .then(b.appear.cmp(&a.appear))
    });
    ret
}

fn main() {
    loop {
        let mut input = String::new();
        println!(
            "Enter seed: (0x?)
输入种子"
        );
        std::io::stdin().read_line(&mut input).unwrap();
        let seed = u32::from_str_radix(&input.trim_start_matches("0x").trim(), 16).unwrap();
        println!(
            "Enter brand by number:
输入品牌编号 
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
输入想要的技能
    0: MainSave 主省,
    1: SubSave 副省,
    2: InkRecovery 回墨,
    3: RunSpeed 走速,
    4: SwimSpeed 游速,
    5: SpecialCharge SP增,
    6: SpecialSave 死亡SP省,
    7: SpecialPower SP强化,
    8: QuickRespawn 复活,
    9: SuperJump 超级跳,
    10: SubPower 副强,
    11: InkResistance 安全鞋,
    12: SubResistance 防爆,
    13: Intensify 行动强化"
        );
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let target = Ability::from(input.trim().parse::<usize>().unwrap());
        println!(
            "Enter lowest standard quality: (ABA,AA,AAA)
输入最低标准品质(ABA,AA,AAA)"
        );
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let standard = Quality::from(input.trim().to_string());
        let sols = search_solution(seed, brand, target, standard);
        if sols.len() == 0 {
            println!("No solution found in 5 steps. 5步内无合适方案");
        } else {
            let mut count = 0;
            println!("Quality 品质\tCost 饮料券消耗\tBegin 序列头\tDrinks 饮料顺序");
            for sol in sols {
                println!("{}", sol);
                count += 1;
                if count >= MAX_DISPLAY {
                    break;
                }
            }
        }
        println!("\nContinue? 继续? (y/n)");
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            break;
        }
    }
}

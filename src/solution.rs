pub struct Solution {
    drink: [Option<Ability>; 5],
    pub cost: usize,
    pub qual: Quality,
    pub appear: usize,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Quality {
    Bad,
    ABA,
    AA,
    AAA,
}
impl From<String> for Quality {
    fn from(x: String) -> Self {
        match x.as_str() {
            "Bad" => Quality::Bad,
            "ABA" => Quality::ABA,
            "AA" => Quality::AA,
            "AAA" => Quality::AAA,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let drink_seq = self
            .drink
            .iter()
            .map(|x| match x {
                Some(x) => format!("{:?}", x),
                None => "None".to_string(),
            })
            .join(", ");
        write!(
            f,
            "Quality: {:?},\tCost: {}, Appear at: {}, Drink: {}",
            self.qual, self.cost, self.appear, drink_seq
        )
    }
}

use itertools::Itertools;

use super::ability::Ability;
use super::PREDICT;
impl From<([Option<Ability>; PREDICT], [Ability; PREDICT], Ability)> for Solution {
    fn from(x: ([Option<Ability>; PREDICT], [Ability; PREDICT], Ability)) -> Self {
        let drinks = x.0;
        let abilities = x.1;
        let target = x.2;
        let mut cost = 0;
        let mut current_drink: Option<Ability> = None;
        for d in drinks.iter() {
            if d != &current_drink && d.is_some() {
                cost += 1;
            }
            current_drink = *d;
        }
        let mut qual = Quality::Bad;
        let mut appear = 0;
        for i in 0..PREDICT - 2 {
            let count = &abilities[i..i + 3]
                .into_iter()
                .filter(|&&a| a == target)
                .count();
            if count == &3 {
                return Solution {
                    drink: drinks,
                    cost,
                    qual: Quality::AAA,
                    appear: i,
                };
            } else if count == &2 {
                if abilities[i + 1] != target {
                    if qual == Quality::Bad {
                        qual = Quality::ABA;
                        appear = i;
                    }
                } else {
                    if qual < Quality::AA {
                        qual = Quality::AA;
                        if abilities[i] == target {
                            appear = i;
                        } else {
                            appear = i + 1;
                        }
                    }
                }
            }
        }
        Solution {
            drink: drinks,
            cost,
            qual,
            appear,
        }
    }
}

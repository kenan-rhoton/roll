extern crate rand;
use rand::Rng;

struct Dice {
    kind: String,
    quantity: u32,
}

struct Roll {
    results: Vec<i32>,
}

struct RollGroup {
    rolls: Vec<Roll>,
}

impl Dice {
    fn roll(&self) -> Roll {
        let mut roll = Roll{results: Vec::new()};
        for _ in 0..self.quantity {
            roll.results.push(roll_kind(&self.kind));
        }
        roll
    }
}

impl RollGroup {
    fn to_string(&self) -> String {
        let mut result = "[".to_string();
        let stringed: Vec<String> = self.rolls.iter().map(|x| x.to_string()).collect();
        result.push_str(&stringed.join("]\n["));
        result.push_str(&"]".to_string());
        result
    }

    fn sum(&self) -> i32 {
        self.rolls.iter().fold(0, |acc, el| acc + el.sum())
    }
}

impl Roll {
    fn to_string(&self) -> String {
        let stringed: Vec<String> = self.results.iter().map(|x| x.to_string()).collect();
        stringed.join(", ")
    }

    fn sum(&self) -> i32 {
        self.results.iter().sum()
    }
}

fn roll_kind(kind: &String) -> i32 {
    let mut rng = rand::thread_rng();
    match kind.parse::<u32>() {
        Ok(size) => rng.gen_range(0, size as i32) + 1,
        Err(_) => {
            match kind.as_ref() {
                "F" => match rng.choose(&[-1, 0, 1]) {
                    Some(&x) => x,
                    None => 0
                },
                _ => 0
            }
        }
    }
}

fn roll(expression: &String) -> Roll {
    let split: Vec<String> = expression.split("d").map(|x| x.to_string()).filter(|x| !x.is_empty()).collect();
    let dice = match split.len() {
        2 => match split[0].parse::<u32>() {
            Ok(num) => Dice{kind: split[1].clone(), quantity: num},
            Err(_) => Dice{kind: split[1].clone(), quantity: 1},
        },
        1 => Dice{kind: split[0].clone(), quantity: 1},
        _ => Dice{kind: "6".to_string(), quantity: 1},
    };
    dice.roll()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d6() {
        for _ in 0..1000 {
            let r = roll(&"d6".to_string());
            assert_eq!(r.results.len(), 1);
            assert!(r.results.iter().all(|&x| x >= 1 && x <= 6));
        }
    }

    #[test]
    fn test_4d10() {
        for _ in 0..1000 {
            let r = roll(&"4d10".to_string());
            assert_eq!(r.results.len(), 4);
            assert!(r.results.iter().all(|&x| x >= 1 && x <= 10), "{:?}", r.results);
        }
    }
}

fn roll_expressions(expressions: Vec<String>) -> RollGroup {
    RollGroup{rolls: expressions.iter().map(|x| roll(x)).collect()}
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: roll [args...]");
    } else {
        let result = roll_expressions(args[1..].to_vec());
        println!("{}", result.to_string());
        println!("{}", result.sum());
    }
}

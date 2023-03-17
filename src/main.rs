use std::error::Error;
use owo_colors::OwoColorize;

use inquire::Text;
use rand::prelude::*;

struct Opts {
    pub n: i32,
    pub bits: i32,
}

impl Opts {
    pub fn inquire() -> Self {
        let lvl: u8 = Text::new("Level:")
            .prompt()
            .unwrap()
            .parse()
            .unwrap();

        let (n, bits) = match lvl {
            1 => (2, 3),
            2 => (4, 3),
            3 => (8, 3),
            4 => (8, 4),
            _ => panic!("Level must be between 1 and 4 (inclusive)"),
        };
        Opts { n, bits }
    }
}

fn game(opts: &Opts) -> Vec<u8> {
    let max = 1 << opts.bits;
    // let dist = Uniform::from(1..max);
    // (0..opts.n).map(|_| thread_rng().sample(dist)).collect()
    (0..opts.n).map(|_| thread_rng().gen_range(1..max)).collect()
}

fn finished(g: &[u8]) -> bool {
    g.iter().all(|&x| x == 0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::inquire();
    let mut g = game(&opts);

    while !finished(&g) {
        let mx = g.iter().max().unwrap();
        for lvl in (0..*mx).rev() {
            for &x in &g {
                if x > lvl {
                    print!("{} ", "▄".bright_green());
                } else {
                    print!("{} ", " ".bright_green());
                }
            }
            println!();
        }
        for i in 0..g.len() {
            print!("{} ", i);
        }
        println!();

        
        let user_answer: u8 = Text::new("Xor:").prompt().unwrap().parse().unwrap();
        let correct_answer = g.iter().fold(0, |acc, &x| acc ^ x);

        if user_answer != correct_answer {
            println!("{}", "Wrong answer!".bright_red());
            println!("Xor was {}", correct_answer.bright_green());
        } else {
            println!("{}", "You got it!".bright_green());
        }

        let move_str = Text::new("Move:").prompt().unwrap();
        let mut tokens = move_str.split_whitespace();
        let index: usize = tokens.next().unwrap().parse()?;
        let amount: u8 = tokens.next().unwrap().parse()?;

        g[index] = amount;
    }

    Ok(())
}

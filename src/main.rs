use std::{collections::HashMap, fmt::Display, io::Write, str::FromStr};

use rand::Rng;

#[derive(Debug)]
enum Item {
    Dice(i8, u8),
    Value(i8),
}

struct Template(Vec<Item>);

fn main() {
    let mut save = HashMap::<String, Template>::new();
    let mut i = 0;
    loop {
        i += 1;

        println!();
        print!("input[{}]: ", i);
        let _ = std::io::stdout().flush();

        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() {
            continue;
        }
        let what = line.split_whitespace().collect::<Vec<_>>();
        if what.is_empty() {
            continue;
        }
        match what[0] {
            "save" => {
                save.insert(what[1].to_string(), what[2].parse::<Template>().unwrap());
            }
            "roll" => {
                if let Some(t) = save.get(what[1]) {
                    println!("{}", t);
                }
            }
            _ => println!("{}", line.parse::<Template>().unwrap()),
        };
    }
}

impl Item {
    /// 计算
    fn cauculate(&self) -> i16 {
        match self {
            Self::Value(v) => *v as i16,
            Self::Dice(n, i) => (0..*n)
                .map(|_| rand::thread_rng().gen_range(1..*i) as i16)
                .sum(),
        }
    }
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.split('d').collect::<Vec<_>>();
        match items.len() {
            1 => match items[0].parse() {
                Ok(v) => Ok(Self::Value(v)),
                Err(_) => Err(()),
            },
            2 => {
                let a = if items[0].is_empty() {
                    1
                } else {
                    match items[0].parse::<i8>() {
                        Ok(v) => v,
                        Err(_) => return Err(()),
                    }
                };
                let b = match items[1].parse::<u8>() {
                    Ok(v) => v,
                    Err(_) => return Err(()),
                };
                Ok(Self::Dice(a, b))
            }
            _ => Err(()),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{}", v),
            Self::Dice(n, i) => write!(f, "{}d{}", n, i),
        }
    }
}

impl Template {
    fn calculate(&self) -> Vec<i16> {
        self.0.iter().map(|i| i.cauculate()).collect()
    }
}

impl FromStr for Template {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .split('+')
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            })
            .filter_map(|s| s.parse::<Item>().ok())
            .collect::<Vec<_>>();

        Ok(Template(items))
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            write!(f, "0")
        } else {
            let values = self.calculate();
            write!(
                f,
                "  {}",
                self.0
                    .iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<_>>()
                    .join(" + ")
            )?;
            write!(
                f,
                "\n= {}",
                values
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(" + ")
            )?;
            write!(f, "\n= {}", values.iter().sum::<i16>())
        }
    }
}

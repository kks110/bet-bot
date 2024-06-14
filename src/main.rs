use rand::Rng;
use std::{thread, time};


fn main() {
    let horses = vec!["Clydesdale", "Shetland Pony", "Shire", "Thoroughbred"];

    let mut race = Race {
        horses: Vec::new(),
        length: 50,
        finished: false,
    };

    for horse_name in &horses {
        let horse = Horse {
            name: horse_name.to_string(),
            max_speed: 5,
            position: 0,
            finisher: false,
        };
        race.horses.push(horse);
    }

    show_race(&race);
    println!("\n\n\n\n");

    let mut rng = rand::thread_rng();

    while !race.finished {
        thread::sleep(time::Duration::new(1, 0));

        for horse in &mut race.horses {
            let roll = rng.gen_range(1..=horse.max_speed);
            let new_position = horse.position + roll;
            horse.position = new_position;
            if new_position >= race.length {
                horse.finisher = true;
                race.finished = true;
            }
        }

        if race.finished {
            if race.finishers().len() == 1 {
                println!("{} wins!", race.finishers()[0].name);
            } else if race.winners().len() > 1 {
                println!("It's a tie between:");
                for winner in race.winners() {
                    println!("{}", winner.name);
                }
            } else {
                println!("It's a photo finish, but: {} wins!", race.winner().name);
            }
        }
        show_race(&race);
        println!("\n\n\n\n");
    }
}

fn show_race(race: &Race) {
    for horse in &race.horses {
        let mut display = String::new();
        for location in 0..race.length {
            if location == horse.position {
                display.push_str("🏇");
            } else {
                display.push_str("-");
            }
        }
        if horse.position >= race.length {
            display.push_str("-🏇");
        } else {
            display.push_str("🏁");
        }

        let mut reversed = display.chars().rev().collect::<String>();

        reversed.push_str(horse.name.as_str());
        println!("{}", reversed);
    }
}

struct Race {
    horses: Vec<Horse>,
    length: i32,
    finished: bool,
}

impl Race {
    fn finishers(&self) -> Vec<&Horse> {
        self.horses.iter().filter(|horse| horse.finisher).collect::<Vec<_>>()
    }

    fn winner(&self) -> &Horse {
        self.finishers().iter().max_by_key(|horse| horse.position).unwrap()
    }

    fn winners(&self) -> Vec<&Horse> {
        self.horses.iter()
            .filter(|&horse| horse.position == self.highest_position())
            .collect()
    }

    fn highest_position(&self) -> i32 {
        self.horses.iter().map(|horse| horse.position).max().unwrap_or(0)
    }
}

struct Horse {
    name: String,
    max_speed: i32,
    position: i32,
    finisher: bool,
}

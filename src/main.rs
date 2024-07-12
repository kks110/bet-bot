use rand::rngs::{OsRng, StdRng};
use rand::*;
use std::{thread, time};
use std::fmt::format;
use poise::{CreateReply, serenity_prelude as serenity};
use rand::SeedableRng;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Start the race
#[poise::command(slash_command)]
async fn begin_race(
    ctx: Context<'_>
) -> Result<(), Error> {
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

    let message = ctx.say(show_race(&race)).await?;

    let mut rng = StdRng::from_seed(OsRng.gen());

    while !race.finished {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        for horse in &mut race.horses {
            let roll = rng.gen_range(1..=horse.max_speed);
            let new_position = horse.position + roll;
            horse.position = new_position;
            if new_position >= race.length {
                horse.finisher = true;
                race.finished = true;
            }
        }

        let mut winner_message = String::new();
        if race.finished {
            if race.finishers().len() == 1 {
                winner_message = format!("{} wins!", race.finishers()[0].name)
            } else if race.winners().len() > 1 {
                let mut tie_message = String::new();
                tie_message.push_str("It's a photo finish, but: ");
                for winner in race.winners() {
                    tie_message.push_str(winner.name.as_str());
                }
                winner_message = tie_message;
            } else {
                winner_message = format!("It's a photo finish, but: {} wins!", race.winner().name);
            }
        }
        let response = format!("{}\n{}", show_race(&race), winner_message);
        message.edit(ctx, CreateReply::default().content(response)).await?;
    }

    Ok(())
}

fn show_race(race: &Race) -> String {
    let mut display = String::new();
    for horse in &race.horses {
        let mut temp_display = String::new();

        for location in 0..race.length {
            if location == horse.position {
                temp_display.push_str("🏇");
            } else {
                temp_display.push_str("-");
            }
        }
        if horse.position >= race.length {
            temp_display.push_str("--🏇");
        } else {
            temp_display.push_str("🏁");
        }

        let mut reversed = temp_display.chars().rev().collect::<String>();
        reversed.push_str(" ");
        reversed.push_str(horse.name.as_str());
        reversed.push_str("\n");
        display.push_str(reversed.as_str());
    }
    return display;
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                begin_race()
            ],
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Running command {}!", ctx.command().qualified_name);
                })
            },
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
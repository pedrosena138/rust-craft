use ::rust_sc2::prelude::{Computer, Difficulty, Race, SC2Result, run_vs_computer};
use std::env;

mod bots;
use bots::base::WorkerRush;

fn main() -> SC2Result<()> {
    env::var("SC2PATH").expect("env variable `SC2PATH` should be set");

    static MAP_NAME: &str = "AcropolisAIE";
    let lauch_options = Default::default();
    let computer = Computer::new(Race::Random, Difficulty::Easy, None);

    let mut bot = WorkerRush::default();
    run_vs_computer(&mut bot, computer, MAP_NAME, lauch_options)
}

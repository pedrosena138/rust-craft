use ::rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct WorkerRush;
impl Player for WorkerRush {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Zerg)
    }

    fn on_start(&mut self) -> SC2Result<()> {
        for worker in &self.units.my.workers {
            worker.attack(Target::Pos(self.enemy_start), false);
        }
        Ok(())
    }
}

fn main() -> SC2Result<()> {
    static MAP_NAME: &str = "AcropolisAIE";
    let lauch_options = Default::default();
    let computer = Computer::new(Race::Random, Difficulty::Easy, None);

    let mut agent = WorkerRush::default();
    run_vs_computer(&mut agent, computer, MAP_NAME, lauch_options)
}

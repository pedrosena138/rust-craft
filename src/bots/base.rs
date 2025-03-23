use ::rust_sc2::prelude::{Player, PlayerSettings, Race, SC2Result, Target, bot};

#[bot]
#[derive(Default)]
pub struct WorkerRush;

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

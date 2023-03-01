use crate::battle_ground::BattleGround;
use crate::render;

#[derive(Clone, Debug)]
pub struct Game {
    size: u8,
    pub player: BattleGround,
    pub enemy: BattleGround,
}

impl Game {
    pub fn new(size: u8) -> Self {
        let player = BattleGround::new(size);
        let enemy = BattleGround::new(size);

        Self::build(player, enemy, size)
    }

    fn build(player: BattleGround, enemy: BattleGround, size: u8) -> Self {
        let game = Game {
            player,
            enemy,
            size
        };

        game
    }

    pub fn render(&self) {
        render::battle_ground(&self.player, self.size);
        // render::battle_ground(&self.enemy, self.size);
    }
}

mod battle_ground;
mod field;
mod ship;
mod render;

use crate::battle_ground::BattleGround;

fn main() {
    let size: u8 = 10;
    let battle_ground: BattleGround = BattleGround::new(size);

    render::battle_ground(battle_ground)
}

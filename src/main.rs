mod battle_ground;
mod field;
mod ship;
mod render;

use crate::battle_ground::BattleGround;

fn main() {
    let battle_ground: BattleGround = BattleGround::new();

    render::battle_ground(battle_ground)
}

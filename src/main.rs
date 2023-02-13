mod battle_ground;
mod field;

use crate::battle_ground::BattleGround;

fn main() {
    let fields: BattleGround = battle_ground::build_battle_ground();

    println!("{:#?}", fields.rows);
}

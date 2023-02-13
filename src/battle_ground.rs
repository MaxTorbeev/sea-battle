use crate::field::Field;

#[derive(Clone, Debug)]
pub struct BattleGround {
    pub rows: Vec<Vec<Field>>,
}

impl BattleGround {
    fn push_field(&mut self, row: Vec<Field>) {
        self.rows.push(row)
    }
}


pub fn build_battle_ground() -> BattleGround{
    let battle_ground_size = 10;

    let mut asics_x: Vec<Field> = vec![];

    let mut battle_ground = BattleGround {
        rows: vec![]
    };

    for x in 1..(battle_ground_size + 1) {
        for y in 1..(battle_ground_size + 1) {
            asics_x.push(Field {
                asics_x: x,
                asics_y: y,
            });

            battle_ground.push_field(asics_x.clone());
        }
    }

    battle_ground
}




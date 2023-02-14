use crate::field::Field;
use crate::ship::Ship;

#[derive(Clone, Debug)]
pub struct BattleGround {
    pub fields: Vec<Field>,
    pub ships: Vec<Ship>
}

impl BattleGround {
    pub fn new() -> Self {
        Self::build()
    }

    fn arrange_ships(&self) -> Vec<Ship> {
        let mut ships = vec![];

        let one_desk_ship = Ship {
            desks: 1,
            fields: vec![(Field { asics_x: 2, asics_y: 3 })],
        };

        ships.push(one_desk_ship);

        ships
    }

    fn build() -> Self {
        let battle_ground_size = 10;

        let mut battle_ground = Self {
            fields: vec![],
            ships: vec![]
        };

        for x in 1..(battle_ground_size + 1) {
            for y in 1..(battle_ground_size + 1) {
                battle_ground.push_field(Field {
                    asics_x: x,
                    asics_y: y
                });
            }
        }

        battle_ground
    }

    fn push_field(&mut self, field: Field) {
        self.fields.push(field)
    }
}



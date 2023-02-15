use crate::field::Field;
use crate::ship::Ship;

#[derive(Clone, Debug)]
pub struct BattleGround {
    pub fields: Vec<Field>,
    pub ships: Vec<Ship>
}

#[derive(Clone, Debug, Default)]
struct Navigator {
    is_left: bool,
    is_right: bool,
    is_up: bool,
    is_down: bool
}

impl BattleGround {
    pub fn new() -> Self {
        Self::build()
    }

    fn get_neighboring_free_field(&self, field: &Field, navigate: Navigator) -> Option<&Field> {
        return if navigate.is_right && navigate.is_up {
            self.fields.iter().find(|f| f.asics_y == field.asics_y - 1 && f.asics_x == field.asics_x + 1)
        } else if navigate.is_left && navigate.is_up {
            self.fields.iter().find(|f| f.asics_y == field.asics_y + 1 && f.asics_x == field.asics_x + 1)
        } else if navigate.is_right && navigate.is_down {
            self.fields.iter().find(|f| f.asics_y == field.asics_y - 1 && f.asics_x == field.asics_x - 1)
        } else if navigate.is_left && navigate.is_down {
            self.fields.iter().find(|f| f.asics_y == field.asics_y + 1 && f.asics_x == field.asics_x - 1)
        } else if navigate.is_left {
            self.fields.iter().find(|f| f.asics_y == field.asics_y + 1 && f.asics_x == field.asics_x)
        } else if navigate.is_right {
            self.fields.iter().find(|f| f.asics_y == field.asics_y - 1 && f.asics_x == field.asics_x)
        } else if navigate.is_up {
            self.fields.iter().find(|f| f.asics_y == field.asics_y && f.asics_x == field.asics_x + 1)
        } else if navigate.is_down {
            self.fields.iter().find(|f| f.asics_y == field.asics_y && f.asics_x == field.asics_x - 1)
        } else {
            self.fields.iter().find(|f| f.asics_x == field.asics_x - 1 && f.asics_y == field.asics_y - 1)
        }
    }

    fn match_arrange_ship(&self, field: &Field, navigator: Navigator) -> bool {
        match self.get_neighboring_free_field(field, navigator) {
            None => false,
            field => {
                if field.unwrap().has_ship(self.ships.clone()) {
                    return true
                }
                false
            }
        }
    }

    pub fn can_not_arrange_ship(&self, field: &Field) -> bool {
        let navigations = vec![
            Navigator { is_right: true, is_up: true, is_left: false, is_down: false },
            Navigator { is_right: true, is_up: false, is_left: false, is_down: true },
            Navigator { is_right: false, is_up: false, is_left: true, is_down: true },
            Navigator { is_right: false, is_up: false, is_left: true, is_down: false },
            Navigator { is_right: true, is_up: false, is_left: false, is_down: false },
            Navigator { is_right: false, is_up: false, is_left: false, is_down: true },
            Navigator { is_right: false, is_up: true, is_left: false, is_down: false },
            Navigator { is_right: false, is_up: true, is_left: true, is_down: false },
            Navigator { is_right: false, is_up: true, is_left: true, is_down: false },
        ];

        for navigator in navigations.iter() {
            if self.match_arrange_ship(field, navigator.clone()) {
                return true;
            }
        }

        false
    }

    fn arrange_ships(&mut self) {
        let mut ships = vec![];

        ships.push(Ship {
            desks: 1,
            fields: vec![(Field { asics_x: 2, asics_y: 3 })],
        });

        ships.push(Ship {
            desks: 1,
            fields: vec![(Field { asics_x: 4, asics_y: 3 })],
        });

        ships.push(Ship {
            desks: 1,
            fields: vec![(Field { asics_x: 1, asics_y: 1 })],
        });

        ships.push(Ship {
            desks: 2,
            fields: vec![
                Field { asics_x: 10, asics_y: 7 },
                Field { asics_x: 10, asics_y: 8 },
            ],
        });

        ships.push(Ship {
            desks: 4,
            fields: vec![
                Field { asics_x: 7, asics_y: 7 },
                Field { asics_x: 7, asics_y: 8 },
                Field { asics_x: 7, asics_y: 9 },
                Field { asics_x: 7, asics_y: 10 },
            ],
        });

        self.ships = ships
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

        battle_ground.arrange_ships();

        battle_ground
    }

    fn push_field(&mut self, field: Field) {
        self.fields.push(field)
    }
}



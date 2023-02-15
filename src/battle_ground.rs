use std::borrow::{Borrow, BorrowMut};
use crate::field::Field;
use crate::ship::Ship;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
struct Navigator {
    is_left: bool,
    is_right: bool,
    is_up: bool,
    is_down: bool
}

#[derive(Clone, Debug)]
pub struct BattleGround {
    pub fields: Vec<Field>,
    pub ships: Vec<Ship>,
    size: u8,
    number_of_ships: HashMap<u8, u8>,
}

impl BattleGround {
    pub fn new(size: u8) -> Self {
        let number_of_ships: HashMap<u8, u8> = HashMap::from([
            (1, 4),
            (2, 3),
            (3, 2),
            (4, 1),
        ]);

        Self::build(size, number_of_ships)
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

    ///
    /// Расположить корабли на поле боя
    ///
    fn arrange_ships(&mut self) {
        for (desks, number) in self.number_of_ships.clone() {
            while self.ships.iter().filter(|ship| ship.desks == desks).count() < number as usize {
                // Расположить двухпалубные корабли
                let mut ship_fields: Vec<Field> = vec![];

                ship_fields = self.arrange_decks_fields(desks, ship_fields).clone();

                self.ships.push(Ship {
                    desks,
                    fields: ship_fields
                });
            }
        }
    }

    ///
    /// Рандомное расположение палуб кораблей на поле боя
    ///
    fn arrange_decks_fields(&mut self, number_of_desc: u8, mut ship_fields: Vec<Field>) -> Vec<Field> {
        let mut rng = rand::thread_rng();

        while ship_fields.len() < number_of_desc as usize {
            let (mut x, mut y) = (rng.gen_range(1..self.size + 1), rng.gen_range(1..self.size + 1));

            if ship_fields.len() > 0 {
                let last_field = ship_fields.last().unwrap();

                (x, y) = (last_field.asics_x + 1, last_field.asics_y);

                if self.can_not_arrange_ship((Field { asics_x: x, asics_y: y }).borrow_mut()) || x > self.size || y > self.size {
                    (x, y) = (last_field.asics_x, last_field.asics_y + 1);
                } else if self.can_not_arrange_ship((Field { asics_x: last_field.asics_x, asics_y: last_field.asics_y + 1 }).borrow_mut()) || last_field.asics_y + 1 > self.size {
                    self.arrange_decks_fields(number_of_desc, vec![]);
                }
            }

            // Если координаты выходят за рамки, то пересобрать весь вектор
            if x > self.size || y > self.size {
                self.arrange_decks_fields(number_of_desc, vec![]);
            }

            let field = Field {
                asics_x: x,
                asics_y: y,
            };

            if !self.can_not_arrange_ship(field.borrow()) {
                ship_fields.push(field);

                ship_fields = self.arrange_decks_fields(number_of_desc, ship_fields);
            } else {
                ship_fields = self.arrange_decks_fields(number_of_desc, vec![]);
            }
        }

        ship_fields
    }

    fn build(size: u8, number_of_ships: HashMap<u8, u8>) -> Self {
        let battle_ground_size = size;

        let mut battle_ground = Self {
            size: battle_ground_size,
            number_of_ships,
            fields: vec![],
            ships: vec![],
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



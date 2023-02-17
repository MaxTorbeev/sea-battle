use crate::ship::Ship;

#[derive(Clone, Debug)]
pub struct Field {
    pub asics_x: u8,
    pub asics_y: u8,
}

impl Field {
    pub fn get_ship<'a>(&'a self, ships: &'a Vec<Ship>) -> Option<&Ship> {
        for ship in ships.iter() {
            for ship_field in ship.fields.iter() {
                if ship_field.asics_y == self.asics_y && ship_field.asics_x == self.asics_x {
                    return Some(ship);
                }
            }
        }

        None
    }
}

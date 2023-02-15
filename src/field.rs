use crate::ship::Ship;

#[derive(Clone, Debug)]
pub struct Field {
    pub asics_x: u8,
    pub asics_y: u8,
}

impl Field {
    pub fn has_ship(&self, ships: Vec<Ship>) -> bool {
        for ship in ships.iter() {
            for shipField in ship.fields.iter() {
                if shipField.asics_y == self.asics_y && shipField.asics_y == self.asics_x {
                    return true;
                }
            }
        }

        false
        // ships.contains(|ship: Ship| ship.fields.contains() )
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub asics_x: u8,
    pub asics_y: u8,
}

impl Field {
    pub fn has_ship() -> bool {
        false
    }
}

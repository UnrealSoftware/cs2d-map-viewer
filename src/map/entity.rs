#[derive(Debug, Clone)]
pub struct Entity {
    pub entity_type: u8,
    pub x: u32,
    pub y: u32,
    pub attributes: Vec<i32>,
    // Add other properties defined in the format here (e.g., strings, triggers)
}

impl Entity {
    pub fn new(entity_type: u8, x: u32, y: u32, attributes: Vec<i32>) -> Self {
        Self {
            entity_type,
            x,
            y,
            attributes,
        }
    }
}
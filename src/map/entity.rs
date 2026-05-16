use macroquad::prelude::*;
use crate::map::entity_type::EntityType;

#[derive(Debug, Clone, Default)]
pub struct Entity {
    // Main Data
    pub entity_type: EntityType,
    pub position: IVec2,
    pub state: u8,
    pub name: String,
    pub trigger: String,

    // Values
    pub ints: [i32; 10],
    pub strings: [String; 10],

    // Reource
    // todo

    pub rotation: f32,
    pub audio_id: u32,
    pub timer: f32,
    pub ai: i32,

    // todo light engine


}

impl Entity {
    pub fn new(entity_type: EntityType, position: IVec2, name: String, trigger: String, ints: [i32; 10], strings: [String; 10]) -> Self {
        Self {
            entity_type,
            position,
            state: 0,
            name,
            trigger,
            ints,
            strings,
            ..Default::default()
        }
    }
}
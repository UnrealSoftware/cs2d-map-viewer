use std::io::{self, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use macroquad::math::U16Vec2;
use crate::map::header::MapHeader;
use crate::map::background::MapBackground;
use crate::map::entity::Entity;

#[derive(Debug, Default)]
pub struct Map {
    pub path: String,
    pub header: MapHeader,
    pub background: MapBackground,

    pub size: U16Vec2,

    pub tiles: Vec<u8>,
    pub shadows: Vec<u8>,
    pub entity: Vec<u8>,
    pub modifiers: Vec<u8>,

    pub entities: Vec<Entity>,
}

impl Map {
    pub fn draw() {

    }
}
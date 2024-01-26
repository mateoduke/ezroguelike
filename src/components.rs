pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,    //helper from bracket-lib to store bg & fg color
    pub glyph: FontCharType, //store a single character or gliph
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; //serves as tag to define entity as the player

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

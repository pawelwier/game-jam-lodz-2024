use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AreaType {
    Empty,
    Available,
    Forbidden,
    Used
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AreaId {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight
}

#[derive(Clone, Component, Copy,Debug)]
pub struct Area {
    pub id: AreaId,
    pub area_type: AreaType,
}
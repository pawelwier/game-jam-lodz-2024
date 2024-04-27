use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AreaType {
    Empty,
    Available,
    Forbidden,
    Used
}

#[derive(Clone, Copy, PartialEq)]
pub enum AreaId {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight
}

#[derive(Clone, Component, Copy)]
pub struct Area {
    pub id: AreaId,
    pub area_type: AreaType,
}
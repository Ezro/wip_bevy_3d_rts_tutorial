use bevy::prelude::*;

pub struct SelectionAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SelectionPending {
    pub begin_pos_ui: Vec2,
    pub end_pos_ui: Vec2,
}

#[derive(PartialEq, Clone)]
pub enum Selection {
    Hover(Option<Entity>),
    OnGoing(SelectionPending),
}

pub struct SelectionRectVisualRes {
    pub data: Entity,
    pub ui_visual: Entity,
}
pub struct SelectionRectParent {
    pub parent_to_visuals: Entity,
}

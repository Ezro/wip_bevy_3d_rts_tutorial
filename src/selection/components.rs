use bevy::prelude::*;

#[derive(Component)]
pub struct SelectionCamera;

#[derive(Component, Reflect, Default)]
pub struct Selectable {
    pub is_selected: bool,
}

#[derive(Component, Reflect, Default)]
pub struct SelectionBox {
    pub origin: Vec3,
    pub half_extents: Vec3,
}
#[derive(Component, Default)]
pub struct SelectionBoxPointsWorld {
    pub points: [Vec3; 8],
}

#[derive(Component, Default)]
pub(super) struct SelectionRectScreen {
    pub(super) min: Vec2,
    pub(super) max: Vec2,
}
#[derive(Component)]
pub struct SelectionRectVisualComponent {
    pub visual: Entity,
}

#[derive(Component)]
pub struct SelectionVisual;

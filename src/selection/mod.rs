use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

mod components;
mod helpers;
mod resources;

pub use components::*;
pub use resources::*;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugLinesPlugin::default());

        app.add_startup_system(create_ui);

        app.add_system(selection_add_components);

        app.add_system(selection_box_update_world);
        app.add_system(convert_box_to_rect.after(selection_box_update_world));
        app.add_system(selection_ui_rect_data_update);

        app.add_system(
            selection_system
                .after(selection_ui_rect_data_update)
                .after(convert_box_to_rect),
        );

        app.add_system(
            rect_visual_ui_update
                .after(selection_ui_rect_data_update)
                .after(convert_box_to_rect),
        );
        app.add_system(selection_ui_visual_toggle.after(rect_visual_ui_update));

        app.add_system(selection_box_show_world.after(selection_box_update_world));
    }
}

pub(super) fn create_ui(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    commands.insert_resource(Selection::Hover(None));
    commands.spawn_bundle(UiCameraBundle::default());
    // root node
    let commands_parent = commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    });
    let parent_to_ui_visuals = commands_parent.id();
    let visual = create_ui_rect_visual(commands_parent);

    commands.insert_resource(SelectionRectParent {
        parent_to_visuals: parent_to_ui_visuals,
    });
    let data = commands
        .spawn()
        .insert(SelectionRectVisualComponent { visual })
        .insert(SelectionRectScreen::default())
        .id();
    commands.insert_resource(SelectionRectVisualRes {
        ui_visual: visual,
        data,
    });
    commands.insert_resource(SelectionAssets {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..default()
        }),
    });
}

fn create_ui_rect_visual(mut commands_parent: bevy::ecs::system::EntityCommands) -> Entity {
    let mut selection_rect_visual = None;
    commands_parent.with_children(|parent| {
        let visual_entity = parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                    border: Rect::all(Val::Px(2.0)),
                    position: Rect {
                        left: Val::Px(600.0),
                        bottom: Val::Px(180.0),
                        ..Default::default()
                    },
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                color: Color::rgba(0.15, 0.65, 0.15, 0.5).into(),
                ..Default::default()
            })
            .id();
        selection_rect_visual = Some(visual_entity);
    });
    selection_rect_visual.unwrap()
}

fn selection_add_components(
    selection_assets: Res<SelectionAssets>,
    selection_parent: Res<SelectionRectParent>,
    mut commands: Commands,
    query: Query<(Entity, &SelectionBox), Without<SelectionRectScreen>>,
) {
    for (e, selection_box) in query.iter() {
        let visual = create_ui_rect_visual(commands.entity(selection_parent.parent_to_visuals));

        commands
            .entity(e)
            .with_children(|parent| {
                parent
                    .spawn_bundle(PbrBundle {
                        mesh: selection_assets.mesh.clone(),
                        material: selection_assets.material.clone(),
                        transform: Transform::default().with_scale(Vec3::new(2.0, 0.1, 2.0)),
                        ..default()
                    })
                    .insert(SelectionVisual);
            })
            .insert(SelectionRectVisualComponent { visual })
            .insert(SelectionRectScreen {
                min: Vec2::new(266.0, 275.0),
                max: Vec2::new(500.0, 550.0),
            })
            .insert(SelectionBoxPointsWorld::default());
    }
}

fn convert_box_to_rect(
    mut commands: Commands,
    windows: Res<Windows>,
    images: Res<Assets<Image>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<SelectionCamera>>,
    mut query: Query<
        (
            Entity,
            &GlobalTransform,
            &SelectionBoxPointsWorld,
            &mut SelectionRectScreen,
        ),
        With<Selectable>,
    >,
) {
    let (cam, cam_transform) = camera_query.get_single().unwrap();
    for (e, g_transform, selection_box, mut rect) in query.iter_mut() {
        let screen_points = selection_box
            .points
            .iter()
            .map(|p| {
                cam.world_to_screen(&windows, &images, cam_transform, *p)
                    .unwrap()
            })
            .collect::<Vec<Vec2>>();
        let (min, max) = get_min_max(screen_points);
        rect.min = Vec2::new(min.0, min.1);
        rect.max = Vec2::new(max.0, max.1);
    }
}

fn get_min_max(points: Vec<Vec2>) -> ((f32, f32), (f32, f32)) {
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for p in points {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }
    ((min_x, min_y), (max_x, max_y))
}

fn selection_box_update_world(
    query: Query<(Entity, &GlobalTransform, &SelectionBox, &Selectable)>,
    mut queryBoxGlobal: Query<&mut SelectionBoxPointsWorld>,
) {
    for (e, g_transform, selection_box, selectable) in query.iter() {
        let center = g_transform.translation + g_transform.rotation * selection_box.origin;
        let size = 1.0;
        let points = [
            center
                + g_transform.rotation
                    * (Vec3::new(-size, size, -size) * selection_box.half_extents),
            center
                + g_transform.rotation
                    * (Vec3::new(size, size, -size) * selection_box.half_extents),
            center
                + g_transform.rotation
                    * (Vec3::new(size, -size, -size) * selection_box.half_extents),
            center
                + g_transform.rotation
                    * (Vec3::new(-size, -size, -size) * selection_box.half_extents),
            center
                + g_transform.rotation
                    * (Vec3::new(-size, size, size) * selection_box.half_extents),
            center
                + g_transform.rotation * (Vec3::new(size, size, size) * selection_box.half_extents),
            center
                + g_transform.rotation
                    * (Vec3::new(size, -size, size) * selection_box.half_extents),
            center
                + g_transform.rotation
                    * (Vec3::new(-size, -size, size) * selection_box.half_extents),
        ];
        if let Ok(mut box_global) = queryBoxGlobal.get_component_mut::<SelectionBoxPointsWorld>(e) {
            box_global.points = points;
        }
    }
}

fn selection_box_show_world(
    mut lines: ResMut<DebugLines>,
    query: Query<(
        Entity,
        &GlobalTransform,
        &SelectionBoxPointsWorld,
        &Selectable,
    )>,
) {
    for (e, g_transform, selection_box_world, selectable) in query.iter() {
        let size = 1.0;
        let points = &selection_box_world.points;
        for i in [0, 4] {
            lines.line(points[i + 0], points[i + 1], 0.0);
            lines.line(points[i + 1], points[i + 2], 0.0);
            lines.line(points[i + 2], points[i + 3], 0.0);
            lines.line(points[i + 3], points[i + 0], 0.0);
        }
        for i in 0..4 {
            lines.line(points[i], points[i + 4], 0.0);
        }
    }
}

use helpers::*;

fn selection_system(
    windows: Res<Windows>,
    mut selection: ResMut<Selection>,
    mouse_button: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &mut Selectable, &SelectionRectScreen)>,
) {
    let window = windows.get_primary().unwrap();
    let ui_position = window.cursor_position();
    if ui_position.is_none() {
        return;
    }
    let ui_position = window.cursor_position().unwrap();
    if mouse_button.pressed(MouseButton::Left) {
        if matches!(*selection, Selection::Hover(_)) {
            *selection = Selection::OnGoing(SelectionPending {
                begin_pos_ui: ui_position,
                end_pos_ui: ui_position,
            });
        } else if let Selection::OnGoing(on_going) = &mut *selection {
            on_going.end_pos_ui = ui_position;
        }
        return;
    }
    if let Selection::OnGoing(on_going) = &mut *selection {
        for (_, mut s, _) in query.iter_mut() {
            s.is_selected = false;
        }
        for (_, mut a, rect) in query.iter_mut() {
            let c1 = rect.min;
            let c2 = rect.max;
            if helper_rect_in_rect((&c1, &c2), (&on_going.begin_pos_ui, &ui_position)) {
                a.is_selected = true;
            }
        }
    }
    for (e, a, rect) in query.iter_mut() {
        let c1 = rect.min;
        let c2 = rect.max;
        if helper_in_rect(&ui_position, &c1, &c2) {
            *selection = Selection::Hover(Some(e));
            return;
        }
    }
    *selection = Selection::Hover(None);
}

fn selection_ui_rect_data_update(
    rect: Res<SelectionRectVisualRes>,
    selection: Res<Selection>,
    mut q: Query<&mut SelectionRectScreen>,
    mut q_visual: Query<&mut Visibility>,
) {
    if let Selection::OnGoing(selection) = &*selection {
        if let Ok(mut visual) = q.get_component_mut::<SelectionRectScreen>(rect.data) {
            let min_x = f32::min(selection.begin_pos_ui.x, selection.end_pos_ui.x);
            let min_y = f32::min(selection.begin_pos_ui.y, selection.end_pos_ui.y);
            let max_x = f32::max(selection.begin_pos_ui.x, selection.end_pos_ui.x);
            let max_y = f32::max(selection.begin_pos_ui.y, selection.end_pos_ui.y);
            visual.min = Vec2::new(min_x, min_y);
            visual.max = Vec2::new(max_x, max_y);
        }
        if let Ok(mut draw) = q_visual.get_component_mut::<Visibility>(rect.ui_visual) {
            draw.is_visible = true;
        }
    } else if let Ok(mut draw) = q_visual.get_component_mut::<Visibility>(rect.ui_visual) {
        draw.is_visible = false;
    }
}

fn rect_visual_ui_update(
    q: Query<(&SelectionRectScreen, &SelectionRectVisualComponent), Changed<SelectionRectScreen>>,
    mut q_visual: Query<&mut Style>,
) {
    for (rect, visual) in q.iter() {
        if let Ok(mut visual) = q_visual.get_component_mut::<Style>(visual.visual) {
            visual.position = Rect {
                left: Val::Px(rect.min.x),
                bottom: Val::Px(rect.min.y),
                ..Default::default()
            };
            visual.size = Size::new(
                Val::Px(rect.max.x - rect.min.x),
                Val::Px(rect.max.y - rect.min.y),
            );
        }
    }
}

pub fn selection_ui_visual_toggle(
    query_selectables: Query<Changed<Selectable>>,
    mut query_visual: Query<(&mut Transform, &Parent, &mut Visibility), With<SelectionVisual>>,
) {
    for (_, parent, mut visibility) in query_visual.iter_mut() {
        if let Ok(selectable) = query_selectables.get_component::<Selectable>(parent.0) {
            if selectable.is_selected {
                visibility.is_visible = true;
            } else {
                visibility.is_visible = false;
            }
        }
    }
}

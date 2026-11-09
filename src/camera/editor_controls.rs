use bevy::{
    camera::RenderTarget,
    prelude::*,
    window::{PrimaryWindow, WindowRef},
};
use leafwing_input_manager::action_state::ActionState;

use crate::{camera::MainCamera, input::ViewAction};

pub fn move_editor_camera(
    time: Res<Time>,
    mut cameras: Query<(&ActionState<ViewAction>, &Projection, &mut Transform), With<MainCamera>>,
) {
    cameras
        .iter_mut()
        .for_each(|(actions, projection, mut transform)| {
            let Projection::Orthographic(ref projection) = *projection else {
                return;
            };

            let move_velocity = actions.axis_pair(&ViewAction::MoveCamera);
            let movement_delta = move_velocity * time.delta_secs() * projection.scale;

            transform.translation.x += movement_delta.x;
            transform.translation.y += movement_delta.y;
        });
}

/// A component attached to camera entities that holds the id of an
/// active `handle_mouse_pan` observer. It is None if there is no
/// such observer.
#[derive(Component, Default)]
pub struct HandleMousePanObserver(Option<Entity>);

pub fn activate_mouse_pan(
    mut commands: Commands,
    mut cameras: Query<
        (
            &ActionState<ViewAction>,
            &RenderTarget,
            &mut HandleMousePanObserver,
        ),
        With<MainCamera>,
    >,
    primary_window: Single<Entity, With<PrimaryWindow>>,
) {
    cameras
        .iter_mut()
        .for_each(|(actions, render_target, mut observer)| {
            let RenderTarget::Window(window) = render_target else {
                return;
            };

            let window_entity = match window {
                WindowRef::Primary => primary_window.entity(),
                WindowRef::Entity(entity) => *entity,
            };

            if actions.just_pressed(&ViewAction::PanCamera) {
                let observer_id = commands
                    .spawn(Observer::new(handle_mouse_pan).with_entity(window_entity))
                    .id();

                observer.0 = Some(observer_id);
            }

            if actions.just_released(&ViewAction::PanCamera)
                && let Some(observer_entity) = observer.0.take()
            {
                commands.entity(observer_entity).despawn();
            }
        });
}

fn handle_mouse_pan(
    drag: On<PointerDrag>,
    mut pan_cameras: Query<(&Camera, &GlobalTransform, &mut Transform), With<MainCamera>>,
) {
    for (camera, global_transform, mut transform) in pan_cameras.iter_mut() {
        let Ok(camera_screen_position) =
            camera.world_to_viewport(global_transform, transform.translation)
        else {
            continue;
        };

        let offset_camera_screen_position = camera_screen_position + drag.delta * -1.; // inverted feels more natural

        let Ok(new_camera_position) =
            camera.viewport_to_world_2d(global_transform, offset_camera_screen_position)
        else {
            continue;
        };

        transform.translation = new_camera_position.extend(transform.translation.z);
    }
}

mod editor_controls;
mod stars;

use avian2d::dynamics::rigid_body::mass_properties::components::ComputedCenterOfMass;
use bevy::{prelude::*, sprite_render::Material2dPlugin};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap};

use crate::{GameState, camera::stars::StarfieldMaterial, input::ViewAction};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<StarfieldMaterial>::default());
        app.add_systems(Update, stars::update_material);
        app.add_systems(
            PostUpdate,
            (
                (
                    camera_follow_target.run_if(in_state(GameState::Game)),
                    (
                        editor_controls::move_editor_camera,
                        editor_controls::activate_mouse_pan,
                    )
                        .run_if(in_state(GameState::ShipEditor)),
                    zoom_camera,
                ),
                stars::update_starfield,
            )
                .chain()
                .after(TransformSystems::Propagate),
        );
    }
}

pub fn camera_backdrop() -> impl Scene {
    bsn! {
        Children [
            #Starfield
            Transform::from_xyz(0.0, 0.0, -100.0)
            Mesh2d(asset_value(Rectangle::default()))
            MeshMaterial2d<StarfieldMaterial>(asset_value(StarfieldMaterial::default()))
        ]
    }
}

#[derive(SceneComponent, FromTemplate, Debug, Clone)]
#[require(Camera2d, InputMap<ViewAction>, editor_controls::HandleMousePanObserver)]
pub struct MainCamera {
    #[entities]
    #[template(built_in)]
    pub follow: Option<Entity>,
    pub min_zoom: f32,
    pub max_zoom: f32,
}

impl MainCamera {
    fn scene() -> impl Scene {
        bsn! {
            #MainCamera
            @camera_backdrop()
        }
    }
}

fn camera_follow_target(
    mut cameras: Query<(&MainCamera, &mut Transform)>,
    targets: Query<(&GlobalTransform, Option<&ComputedCenterOfMass>), Without<MainCamera>>,
) {
    cameras.iter_mut().for_each(|(camera, mut transform)| {
        let Some(follow_target) = camera.follow else {
            return;
        };

        let Ok((target_transform, center_of_mass)) = targets.get(follow_target) else {
            return;
        };

        let mut center = target_transform.translation().truncate();

        if let Some(center_of_mass) = center_of_mass {
            center += center_of_mass
                .0
                .rotate_angle(target_transform.rotation().to_euler(EulerRot::ZXY).0);
        }

        transform.translation.x = center.x;
        transform.translation.y = center.y;
    });
}

fn zoom_camera(mut cameras: Query<(&ActionState<ViewAction>, &mut Projection, &MainCamera)>) {
    cameras
        .iter_mut()
        .for_each(|(actions, mut projection, camera_settings)| {
            let Projection::Orthographic(ref mut projection) = *projection else {
                return;
            };

            let zoom_rate: f32 = 0.05;
            let zoom_amount = actions.value(&ViewAction::ZoomCamera);

            projection.scale *= 1.0 - zoom_amount * zoom_rate;
            projection.scale = projection
                .scale
                .clamp(camera_settings.min_zoom, camera_settings.max_zoom)
        });
}

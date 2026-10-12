use bevy::{
    ecs::relationship::Relationship,
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<StarfieldMaterial>::default());
        app.add_systems(Update, update_material);
        app.add_systems(
            PostUpdate,
            (update_camera, update_starfield).after(TransformSystems::Propagate),
        );
    }
}

#[derive(SceneComponent, FromTemplate, Debug, Clone)]
#[require(Camera2d)]
pub struct MainCamera {
    pub follow: Entity,
}

impl MainCamera {
    fn scene() -> impl Scene {
        bsn! {
            #MainCamera
            Children [
                Mesh2d(asset_value(Rectangle::default()))
                MeshMaterial2d<StarfieldMaterial>(asset_value(StarfieldMaterial::default()))
            ]
        }
    }
}

fn update_camera(
    mut cameras: Query<(&MainCamera, &mut Transform)>,
    entities: Query<&GlobalTransform, Without<MainCamera>>,
) {
    cameras.iter_mut().for_each(|(camera, mut transform)| {
        let Ok(target) = entities.get(camera.follow) else {
            return;
        };

        transform.translation.x = target.translation().x;
        transform.translation.y = target.translation().y;
    });
}

fn update_starfield(
    mut starfields: Query<(&ChildOf, &mut Transform)>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    starfields
        .iter_mut()
        .for_each(|(starfield, mut transform)| {
            let Ok((camera, camera_transform)) = cameras.get(starfield.get()) else {
                return;
            };

            let Some(world_min) = camera
                .ndc_to_world(camera_transform, Vec3::new(-1.0, -1.0, 1.0))
                .map(Vec3::truncate)
            else {
                return;
            };

            let Some(world_max) = camera
                .ndc_to_world(camera_transform, Vec3::new(1.0, 1.0, 1.0))
                .map(Vec3::truncate)
            else {
                return;
            };

            let world_size = (world_max - world_min).abs();

            // Scale assuming a default 1x1 quad mesh unit size
            // Idk ts isn't updating properly every frame so just scale it :sob:
            transform.scale.x = world_size.x;
            transform.scale.y = world_size.y;
        });
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
struct StarfieldMaterial {
    // On webgl non-16 byte allignment causes an error
    #[uniform(0)]
    time: Vec4,
}

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/starfield.wesl".into()
    }
}

fn update_material(time: Res<Time>, mut materials: ResMut<Assets<StarfieldMaterial>>) {
    materials.iter_mut().for_each(|(_, material)| {
        material.time = Vec4::new(time.elapsed_secs(), 0.0, 0.0, 0.0);
    });
}

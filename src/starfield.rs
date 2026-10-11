use bevy::{
    ecs::relationship::Relationship,
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

pub struct StarfieldMaterialPlugin;

impl Plugin for StarfieldMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<StarfieldMaterial>::default());
        app.add_systems(Update, (update_material, update_starfield));
    }
}

#[derive(Component)]
#[require(Mesh2d)]
#[relationship(relationship_target = StarfieldCamera)]
pub struct StarfieldFor(Entity);

#[derive(Component)]
#[relationship_target(relationship = StarfieldFor)]
pub struct StarfieldCamera(Vec<Entity>);

fn update_starfield(
    mut starfields: Query<(&StarfieldFor, &mut Transform)>,
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

            // Center the mesh on the camera's position
            transform.translation.x = camera_transform.translation().x;
            transform.translation.y = camera_transform.translation().y;

            // Scale assuming a default 1x1 quad mesh unit size
            transform.scale.x = world_size.x;
            transform.scale.y = world_size.y;
        });
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct StarfieldMaterial {
    #[uniform(0)]
    time: f32,
}

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/starfield.wesl".into()
    }
}

fn update_material(time: Res<Time>, mut materials: ResMut<Assets<StarfieldMaterial>>) {
    materials.iter_mut().for_each(|(_, material)| {
        material.time = time.elapsed_secs();
    });
}

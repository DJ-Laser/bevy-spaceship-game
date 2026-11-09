use bevy::{
    prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::Material2d,
};

pub fn update_starfield(
    cameras: Query<(&Children, &Camera, &Projection)>,
    mut starfields: Query<&mut Transform, With<MeshMaterial2d<StarfieldMaterial>>>,
) {
    cameras.iter().for_each(|(children, camera, projection)| {
        let Some(logical_size) = camera.logical_viewport_size() else {
            return;
        };

        let Projection::Orthographic(ortho_proj) = projection else {
            return;
        };

        let view_size = logical_size * ortho_proj.scale;

        let mut iter = starfields.iter_many_mut(children).matched();
        while let Some(mut transform) = iter.fetch_next() {
            // Scale assuming a default 1x1 quad mesh unit size
            transform.scale.x = view_size.x;
            transform.scale.y = view_size.y;
        }
    });
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct StarfieldMaterial {
    // On webgl non-16 byte allignment causes an error
    #[uniform(0)]
    time: Vec4,
}

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/starfield.wesl".into()
    }
}

pub fn update_material(time: Res<Time>, mut materials: ResMut<Assets<StarfieldMaterial>>) {
    materials.iter_mut().for_each(|(_, material)| {
        material.time = Vec4::new(time.elapsed_secs(), 0.0, 0.0, 0.0);
    });
}

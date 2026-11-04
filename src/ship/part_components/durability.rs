use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Durability(pub f32);

#[derive(Component, Default, Clone)]
pub struct MaxDurability(pub f32);

pub fn init_part_durability(
    mut commands: Commands,
    mut hulls: Query<(Entity, &MaxDurability), (Added<MaxDurability>, Without<Durability>)>,
) {
    hulls.iter_mut().for_each(|(entity, max)| {
        commands.entity(entity).insert(Durability(max.0));
    });
}

pub fn remove_broken_parts(mut commands: Commands, hulls: Query<(Entity, &Durability)>) {
    hulls.iter().for_each(|(entity, durability)| {
        if durability.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    });
}

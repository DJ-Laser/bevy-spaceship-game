use avian2d::prelude::*;
use bevy::{ecs::relationship::Relationship as _, prelude::*};
use leafwing_input_manager::{Actionlike, action_state::ActionState};

use crate::ship::{
    Spaceship,
    part_components::{InactivePart, power::PowerConsumer},
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum WeaponAction {
    Fire,
}

#[derive(Component, Clone, Default)]
#[require(PowerConsumer, ActionState<WeaponAction>, Transform)]
pub struct ProjectileWeapon {
    pub velocity: f32,
    pub frequency: f32,
    /// Power draw / sec needed for maximum fire rate
    pub power_draw: f32,

    fire_cooldown: f32,
}

pub fn request_weapon_power(
    mut weapons: Query<(&ProjectileWeapon, &mut PowerConsumer), Without<InactivePart>>,
) {
    weapons.iter_mut().for_each(|(weapon, mut consumer)| {
        if weapon.fire_cooldown > 0.0 {
            consumer.request_power(weapon.power_draw);
        }
    });
}

pub fn fire_weapons(
    time: Res<Time>,
    mut commands: Commands,
    spaceships: Query<(&Position, &Rotation, &LinearVelocity), With<Spaceship>>,
    mut weapons: Query<
        (
            &ChildOf,
            &mut ProjectileWeapon,
            &ActionState<WeaponAction>,
            &mut PowerConsumer,
        ),
        Without<InactivePart>,
    >,
) {
    weapons
        .iter_mut()
        .for_each(|(p, mut weapon, actions, consumer)| {
            if weapon.fire_cooldown > 0.0 {
                weapon.fire_cooldown -= time.delta_secs() * consumer.allocated_fraction();
            }

            if weapon.fire_cooldown <= 0.0 && actions.pressed(&WeaponAction::Fire) {
                let Ok((position, rotation, linear_velocity)) = spaceships.get(p.get()) else {
                    return;
                };

                let velocity = linear_velocity.0 + rotation * Vec2::Y * weapon.velocity;

                weapon.fire_cooldown = 1.0 / weapon.frequency;

                let circle = Circle::new(2.5);
                commands
                    .queue_spawn_scene(bsn! {
                        Position({position.0})
                        LinearVelocity(velocity)
                        RigidBody
                        Collider::from(circle)
                        ColliderDensity(1.0)
                        CollisionEventsEnabled
                        Projectile {
                            owner: {p.get()}
                            damage: 10.0,
                        }
                        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 1.0, 0.0)))
                        Mesh2d(asset_value(circle))
                    })
                    .observe(apply_projectile_damage);
            }
        });
}

#[derive(Component, Default, Clone)]
pub struct ShipHull {
    pub max_hp: f32,
    pub current_hp: f32,
}

pub fn init_ship_hull(mut hulls: Query<&mut ShipHull, Added<ShipHull>>) {
    hulls.iter_mut().for_each(|mut hull| {
        hull.current_hp = hull.max_hp;
    });
}

pub fn remove_broken_hulls(mut commands: Commands, hulls: Query<(Entity, &ShipHull)>) {
    hulls.iter().for_each(|(entity, hull)| {
        if hull.current_hp <= 0.0 {
            commands.entity(entity).despawn();
        }
    });
}

#[derive(Component, Clone, FromTemplate)]
pub struct Projectile {
    owner: Entity,
    damage: f32,
}

fn apply_projectile_damage(
    event: On<CollisionStart>,
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile)>,
    mut hulls: Query<(Entity, Option<&ChildOf>, &mut ShipHull)>,
) {
    let Ok((projectile_id, projectile)) = projectiles.get(event.collider1) else {
        return;
    };

    let Ok((hull_id, p, mut hull)) = hulls.get_mut(event.collider2) else {
        return;
    };

    if projectile.owner == hull_id {
        return;
    } else if let Some(p) = p
        && projectile.owner == p.get()
    {
        return;
    }

    hull.current_hp -= projectile.damage;
    commands.entity(projectile_id).despawn();
}

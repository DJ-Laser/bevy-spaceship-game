use std::f32::consts::FRAC_PI_2;

use avian2d::prelude::*;
use bevy::{ecs::relationship::Relationship as _, prelude::*};
use leafwing_input_manager::{Actionlike, action_state::ActionState};

use crate::ship::{
    Spaceship,
    part_components::{Durability, InactivePart, power::PowerConsumer},
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum WeaponAction {
    Fire,
}

#[derive(Component, Clone, Default)]
pub struct ProjectileWeaponState {
    fire_cooldown: f32,
    last_spawn_point: usize,
}

#[derive(Component, Clone, Default)]
#[require(ProjectileWeaponState, PowerConsumer, ActionState<WeaponAction>, Transform)]
pub struct ProjectileWeapon {
    pub velocity: f32,
    pub frequency: f32,
    /// Power draw / sec needed for maximum fire rate
    pub power_draw: f32,
}

#[derive(Component, Clone, Default)]
pub struct ProjectileSpawnPoint;

pub fn request_weapon_power(
    mut weapons: Query<
        (
            &ProjectileWeapon,
            &ProjectileWeaponState,
            &mut PowerConsumer,
        ),
        Without<InactivePart>,
    >,
) {
    weapons
        .iter_mut()
        .for_each(|(weapon, state, mut consumer)| {
            if state.fire_cooldown > 0.0 {
                consumer.request_power(weapon.power_draw);
            }
        });
}

pub fn fire_weapons(
    time: Res<Time>,
    mut commands: Commands,
    spaceships: Query<&LinearVelocity, With<Spaceship>>,
    mut weapons: Query<
        (
            &Children,
            &ChildOf,
            &ProjectileWeapon,
            &mut ProjectileWeaponState,
            &ActionState<WeaponAction>,
            &mut PowerConsumer,
            &GlobalTransform,
        ),
        Without<InactivePart>,
    >,
    spawn_points: Query<&GlobalTransform, With<ProjectileSpawnPoint>>,
) {
    weapons.iter_mut().for_each(
        |(children, parent, weapon, mut state, actions, consumer, weapon_transform)| {
            if state.fire_cooldown > 0.0 {
                state.fire_cooldown -= time.delta_secs() * consumer.allocated_fraction();
            }

            if state.fire_cooldown <= 0.0 && actions.pressed(&WeaponAction::Fire) {
                let Ok(linear_velocity) = spaceships.get(parent.get()) else {
                    return;
                };

                let mut spawn_points = spawn_points.iter_many(children).matched().enumerate();
                let first = spawn_points.next();
                let spawn_point = spawn_points.nth(state.last_spawn_point);
                let (index, spawn_point) = spawn_point.or(first).unwrap_or((0, weapon_transform));
                state.last_spawn_point = index;

                let rotation = spawn_point.rotation().to_euler(EulerRot::XYZ).2;
                let velocity =
                    linear_velocity.0 + Vec2::from_angle(rotation + FRAC_PI_2) * weapon.velocity;

                state.fire_cooldown = 1.0 / weapon.frequency;

                let circle = Circle::new(2.5);
                commands
                    .queue_spawn_scene(bsn! {
                        #Projectile
                        Position({spawn_point.translation().truncate()})
                        LinearVelocity(velocity)
                        RigidBody::Kinematic
                        Collider::from(circle)
                        Sensor
                        CollisionEventsEnabled
                        Projectile {
                            owner: {parent.get()}
                            damage: 10.0,
                        }
                        MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 1.0, 0.0)))
                        Mesh2d(asset_value(circle))
                    })
                    .observe(apply_projectile_damage);
            }
        },
    );
}

#[derive(Component, Clone, FromTemplate)]
pub struct Projectile {
    #[entities]
    owner: Entity,
    damage: f32,
}

fn apply_projectile_damage(
    event: On<CollisionStart>,
    mut commands: Commands,
    projectiles: Query<(Entity, &Projectile)>,
    children: Query<&ChildOf>,
    names: Query<&Name>,
    mut parts: Query<(Entity, &mut Durability)>,
) {
    let Ok((projectile_id, projectile)) = projectiles.get(event.collider1) else {
        return;
    };

    let name = names.get(event.collider2).ok();
    if let Some(name) = name {
        println!("hit {name}");
    } else {
        println!("hit unnamed");
    }

    let Some((part_id, mut durability)) = children
        .iter_ancestors(event.collider2)
        .find(|ancestor| {
            let name = names.get(*ancestor).ok();
            println!(
                "Ancestor {}, {}",
                name.map_or("None".into(), |name| name.to_string()),
                parts.contains(*ancestor)
            );

            parts.contains(*ancestor)
        })
        .and_then(|part_id| parts.get_mut(part_id).ok())
    else {
        println!("Not a part with durability");
        return;
    };

    if projectile.owner == part_id {
        return;
    } else if let Ok(p) = children.get(part_id)
        && projectile.owner == p.get()
    {
        return;
    }

    println!("Damaging {name:?}");
    durability.0 -= projectile.damage;
    commands.entity(projectile_id).try_despawn();
}

use avian2d::prelude::*;
use bevy::{ecs::relationship::Relationship as _, prelude::*};
use leafwing_input_manager::{Actionlike, action_state::ActionState};

use crate::ships::{InactiveSystem, Spaceship, power_system::PowerConsumer};

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
    mut weapons: Query<(&ProjectileWeapon, &mut PowerConsumer), Without<InactiveSystem>>,
) {
    weapons.iter_mut().for_each(|(weapon, mut consumer)| {
        if weapon.fire_cooldown > 0.0 {
            consumer.request_power(weapon.power_draw);
        }
    });
}

#[expect(clippy::type_complexity)]
pub fn fire_weapons(
    time: Res<Time>,
    mut commands: Commands,
    spaceships: Query<&LinearVelocity, With<Spaceship>>,
    mut weapons: Query<
        (
            &ChildOf,
            &mut ProjectileWeapon,
            &ActionState<WeaponAction>,
            &mut PowerConsumer,
            &GlobalTransform,
        ),
        Without<InactiveSystem>,
    >,
) {
    weapons
        .iter_mut()
        .for_each(|(p, mut weapon, actions, consumer, transform)| {
            if weapon.fire_cooldown > 0.0 {
                weapon.fire_cooldown -= time.delta_secs() * consumer.allocated_fraction();
            }

            if weapon.fire_cooldown <= 0.0 && actions.pressed(&WeaponAction::Fire) {
                let Ok(linear_velocity) = spaceships.get(p.get()) else {
                    return;
                };

                let angle_z = transform.rotation().to_euler(EulerRot::XYZ).2;
                let velocity = linear_velocity.0 + Vec2::Y.rotate_angle(angle_z) * weapon.velocity;

                weapon.fire_cooldown = 1.0 / weapon.frequency;
                println!("Fired");
                commands.queue_spawn_scene(bsn! {
                    Position({transform.translation().truncate()})
                    LinearVelocity(velocity)
                    RigidBody
                    Projectile {
                        owner: {p.get()}
                        damage: 10.0,
                    }
                    MeshMaterial2d<ColorMaterial>(asset_value(Color::srgb(1.0, 1.0, 0.0)))
                    Mesh2d(asset_value(Circle::new(2.5)))
                });
            }
        });
}

#[derive(Component, Clone, FromTemplate)]
pub struct Projectile {
    owner: Entity,
    damage: f32,
}

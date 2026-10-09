use bevy::{ecs::component, prelude::*};

#[derive(Component, Default, Clone, Copy)]
struct PrevPhysicsState<T>(T);

// Linear movement components
#[derive(Component, Default, Clone, Copy)]
#[require(PrevPhysicsState<Position>)]
pub struct Position(pub Vec2); // In units

#[derive(Component)]
pub struct Velocity(pub Vec2); // In units per second

// Angular movement components
#[derive(Component, Default, Clone, Copy)]
#[require(PrevPhysicsState<Angle>)]
pub struct Angle(pub f32); // In radians

#[derive(Component)]
pub struct AngularVelocity(pub f32); // In radians per second

fn update_prev_state<T>(mut query: Query<(&T, &mut PrevPhysicsState<T>)>)
where
    T: Component<Mutability = component::Mutable>,
    T: Copy,
{
    query.iter_mut().for_each(|(state, mut prev_state)| {
        prev_state.0 = *state;
    });
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    query.iter_mut().for_each(|(mut position, velocity)| {
        position.0 += velocity.0 * time.delta_secs();
    });
}

fn apply_angular_velocity(time: Res<Time>, mut query: Query<(&mut Angle, &AngularVelocity)>) {
    query.iter_mut().for_each(|(mut angle, angular_velocity)| {
        angle.0 += angular_velocity.0 * time.delta_secs();
    });
}

type PhysicsComponentQuery<'a> = AnyOf<(
    (&'a Position, &'a PrevPhysicsState<Position>),
    (&'a Angle, &'a PrevPhysicsState<Angle>),
)>;

fn transform_interpolation(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(PhysicsComponentQuery, &mut Transform)>,
) {
    query
        .iter_mut()
        .for_each(|((position, angle), mut transform)| {
            let lerp_fraction = fixed_time.overstep_fraction();

            if let Some((position, prev_position)) = position {
                let lerp_position = position.0.lerp(prev_position.0.0, lerp_fraction);

                transform.translation.x = lerp_position.x;
                transform.translation.y = lerp_position.y;
            }

            if let Some((angle, prev_angle)) = angle {
                let lerp_angle = angle.0.lerp(prev_angle.0.0, lerp_fraction);

                transform.rotation = Quat::from_rotation_z(lerp_angle);
            }
        });
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_prev_state::<Position>, apply_velocity).chain(),
        );
        app.add_systems(
            FixedUpdate,
            (update_prev_state::<Angle>, apply_angular_velocity).chain(),
        );
        app.add_systems(Update, transform_interpolation);
    }
}

use crate::prelude::*;
use bevy_rapier2d::prelude::*;

use rand::Rng;
use std::time::Duration;

const MOB_COLOUR: Color = Color::GREEN;
const MOB_SPEED: f32 = 150.0;
const MOB_SIZE: f32 = 64.0;
const SPAWN_RADIUS: f32 = 1000.0;

#[derive(Component)]
pub struct Mob;

#[derive(Resource)]
pub struct MobTimer(pub Timer);

#[derive(Resource)]
pub struct MobCount(pub u32);

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, spawn_mobs)
            .add_systems(Update, mob_movement);
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(MobCount(0));
    commands.insert_resource(MobTimer(Timer::from_seconds(0.3, TimerMode::Repeating)));
}

pub fn spawn_mobs(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MobTimer>,
    mut count: ResMut<MobCount>
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let angle: f32 = rand::thread_rng().gen_range(0.0..360.0);
        let pos = Vec3::new(
            angle.cos() * SPAWN_RADIUS,
            angle.sin() * SPAWN_RADIUS,
            0.0);

        let root = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(pos)),
                Name::new("Mob"),
                RigidBody::Dynamic,
                Collider::ball((MOB_SIZE * 0.7)/ 2.),
                GravityScale(0.),
                Velocity::zero(),
                LockedAxes::ROTATION_LOCKED,
                Mob))
            .id();
        count.0 += 1;

    }

}

pub fn mob_movement(
    player_query: Query<&Transform, With<Player>>,
    mut mob_query: Query<(&mut Velocity, &Transform), With<Mob>>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut velocity, transform) in &mut mob_query {
            let mut direction = Vec2::new(
                player_transform.translation.x - transform.translation.x,
                player_transform.translation.y - transform.translation.y
            );

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            velocity.linvel = direction * MOB_SPEED;
        }
    }
}
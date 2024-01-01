use crate::prelude::*;
use bevy_rapier2d::prelude::*;

use std::time::Duration;

const PLAYER_COLOUR: Color = Color::PURPLE;
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub idle_animation: Handle<AnimationClip>,
    pub walk_animation: Handle<AnimationClip>,

}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut animations: ResMut<Assets<AnimationClip>>,
    handles: Res<SpriteAssets>,
) {
    let animation_player = AnimationPlayer::default();
    let body_sprite_name = Name::new("BodySprite");
    let eye_sprite_name = Name::new("EyeSprite");
    let anchor_name = Name::new("PlayerAnchor");
    let eye_anchor_name = Name::new("EyeAnchor");

    let mut idle_animation = AnimationClip::default();
    idle_animation.add_curve_to_path(
        EntityPath {
            parts: vec![anchor_name.clone(), body_sprite_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.5, 1.0],
            keyframes: Keyframes::Scale(vec![
                Vec3::new(1., 1., 1.),
                Vec3::new(1.05, 0.95, 1.),
                Vec3::new(1., 1., 1.),
            ]),
        },
    );
    idle_animation.add_curve_to_path(
        EntityPath {
            parts: vec![anchor_name.clone(), eye_anchor_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.5, 1.0],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(0., 0., 0.),
                Vec3::new(0., 3., 0.),
                Vec3::new(0., 0., 0.),
            ]),
        },
    );
    idle_animation.add_curve_to_path(
        EntityPath {
            parts: vec![anchor_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(0., 0., 0.),
            ]),
        },
    );

    let mut walk_animation = AnimationClip::default();
    walk_animation.add_curve_to_path(
        EntityPath {
            parts: vec![anchor_name.clone(), body_sprite_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.1, 0.3, 0.5],
            keyframes: Keyframes::Scale(vec![
                Vec3::new(0.95, 1.1, 1.),
                Vec3::new(1.1, 0.95, 1.),
                Vec3::new(0.95, 1.1, 1.),
            ]),
        },
    );

    // 0, 8, 0, -8
    walk_animation.add_curve_to_path(
        EntityPath {
            parts: vec![anchor_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.1, 0.2, 0.3, 0.4],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(0., 0., 0.),
                Vec3::new(0., 8., 0.),
                Vec3::new(0., 0., 0.),
                Vec3::new(0., -8., 0.),
                Vec3::new(0., 0., 0.),
            ]),
        },
    );

    walk_animation.add_curve_to_path(
        EntityPath {
            parts: vec![anchor_name.clone(), eye_anchor_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.1, 0.3, 0.5],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(0., 2., 0.),
                Vec3::new(0., -3., 0.),
                Vec3::new(0., 2., 0.),
            ]),
        },
    );

    let mut blink_animation = AnimationClip::default();
    blink_animation.add_curve_to_path(
        EntityPath {
            parts: vec![eye_sprite_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.1, 0.3, 0.5],
            keyframes: Keyframes::Scale(vec![
                Vec3::new(0.95, 1.1, 1.),
                Vec3::new(1.1, 0.95, 1.),
                Vec3::new(0.95, 1.1, 1.),
            ]),
        },
    );

    commands.insert_resource(PlayerAnimations { 
        idle_animation: animations.add(idle_animation),
        walk_animation: animations.add(walk_animation)
    });


    let root = commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Player"),
            RigidBody::Dynamic,
            Collider::ball((PLAYER_SIZE * 0.7)/ 2.),
            ColliderMassProperties::Density(0.0),
            GravityScale(0.),
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
            Player))
        .id();
    let body_anchor = commands
        .spawn((
            SpatialBundle::default(), 
            anchor_name, 
            animation_player,
        ))
        .id();
    let eye_anchor = commands
        .spawn((SpatialBundle::default(), eye_anchor_name))
        .id();
    let body_sprite = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: PLAYER_COLOUR,
                    custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                    ..default()
                },
                texture: handles.player_body.clone(),
                ..default()
            },
            body_sprite_name,
        ))
        .id();
    let eye_sprite = commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::rgba(0., 0., 0., 0.8),
                    index: 0,
                    custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                    ..default()
                },
                texture_atlas: handles.player_eyes.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..default()
            },
            eye_sprite_name,
            AnimationTimer(Timer::from_seconds(2.5, TimerMode::Once)),
            Animations {
                animations: vec![Animation {
                    current_frame: 0,
                    frames: vec![
                        AnimationFrame {
                            index: 0,
                            time: 2.5,
                        },
                        AnimationFrame {
                            index: 1,
                            time: 0.15,
                        },
                    ],
                }],
                current_animation: 0,
            },
        ))
        .id();

    commands.entity(root).push_children(&[body_anchor]);
    commands
        .entity(body_anchor)
        .push_children(&[eye_anchor, body_sprite]);
    commands.entity(eye_anchor).push_children(&[eye_sprite]);
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    animations: Res<PlayerAnimations>,
    mut animation_query: Query<&mut AnimationPlayer>,
) {
    if let Ok(mut velocity) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec2::new(-1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec2::new(1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec2::new(0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec2::new(0.0, -1.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        velocity.linvel = direction * PLAYER_SPEED;

        if let Ok(mut animation_player) = animation_query.get_single_mut() {
            if direction.length() > 0.0 {
                animation_player
                    .play_with_transition(
                        animations.walk_animation.clone_weak(),
                        Duration::from_millis(250)
                    ).repeat();
            } else {
                animation_player
                    .play_with_transition(
                        animations.idle_animation.clone_weak(),
                        Duration::from_millis(250)
                    ).repeat();
            }
        }
    }
}
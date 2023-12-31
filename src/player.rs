use crate::prelude::*;

use bevy::sprite::Anchor;

const PLAYER_COLOUR: Color = Color::PURPLE;
const PLAYER_SPEED: f32 = 300.0;
const PLAYER_SIZE: f32 = 64.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct HeldItem {
    pub item: Option<Entity>,
}

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
    let mut animation_player = AnimationPlayer::default();
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
    walk_animation.add_curve_to_path(
        EntityPath {
            parts: vec![anchor_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.2, 0.4],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(0., 0., 0.),
                Vec3::new(0., -15., 0.),
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

    animation_player
        .play(animations.add(walk_animation))
        .repeat();

    let root = commands
        .spawn((SpatialBundle::default(), Name::new("Player"), Player))
        .id();
    let body_anchor = commands
        .spawn((SpatialBundle::default(), anchor_name, animation_player))
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
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                texture: handles.player_body.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
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
                    anchor: Anchor::BottomCenter,
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
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

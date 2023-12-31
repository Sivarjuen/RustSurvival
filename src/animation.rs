// Courtesy of https://github.com/TanTanDev/flappy_bevy/

use crate::prelude::*;

use std::time::Duration;

pub struct SpriteAnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationFrame {
    pub time: f32,
    pub index: i32,
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub current_frame: i32,
}

#[derive(Component)]
pub struct Animations {
    pub animations: Vec<Animation>,
    pub current_animation: i32,
}

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut Animations,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, mut animations, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let current_animation_index = animations.current_animation;
            if let Some(animation) = animations
                .animations
                .get_mut(current_animation_index as usize)
            {
                animation.current_frame += 1;
                if animation.current_frame as usize >= animation.frames.len() {
                    animation.current_frame = 0;
                }
                if let Some(frame) = animation.frames.get(animation.current_frame as usize) {
                    timer.set_duration(Duration::from_secs_f32(frame.time));
                    timer.reset();
                    sprite.index = frame.index as usize;
                }
            }
        }
    }
}

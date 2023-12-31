use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct SpriteAssets {
    pub player_body: Handle<Image>,
    pub player_eyes: Handle<TextureAtlas>,
}

#[derive(Debug, Resource)]
pub struct SpriteAtlas {}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_textures);
    }
}

fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_eyes_handle = asset_server.load("player/player-eyes.png");
    let player_eyes_atlas =
        TextureAtlas::from_grid(player_eyes_handle, Vec2::new(256., 256.), 2, 1, None, None);

    commands.insert_resource(SpriteAssets {
        player_body: asset_server.load("player/player-body.png"),
        player_eyes: atlases.add(player_eyes_atlas),
    });
}

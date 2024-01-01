mod animation;
mod assets;
mod fps;
mod mob;
mod physics;
mod player;
mod ui;

mod prelude {
    pub use crate::animation::*;
    pub use crate::assets::*;
    pub use crate::fps::*;
    pub use crate::mob::*;
    pub use crate::physics::*;
    pub use crate::player::*;
    pub use crate::ui::*;
    pub use bevy::prelude::*;
}

use crate::prelude::*;
use bevy_editor_pls::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

const BG_COLOUR: Color = Color::rgb(0.06, 0.06, 0.06);

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOUR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Geometry Survival".to_string(),
                resolution: (1920.0, 1080.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            EditorPlugin::default(),
            // WorldInspectorPlugin::new()
            CustomUiPlugin,
            FpsPlugin,
            PhysicsPlugin,
            SpriteAnimationPlugin,
            AssetsPlugin,
            PlayerPlugin,
            MobPlugin,
            
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

use crate::prelude::*;

#[derive(Component)]
pub struct MobCountText;

pub struct CustomUiPlugin;

impl Plugin for CustomUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_mob_counter);
    }
}

pub fn setup(mut commands: Commands) {
    let root = commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::BLACK.with_a(0.3)),
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.),
                right: Val::Px(5.),
                top: Val::Auto,
                left: Val::Auto,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            ..default()
        })
        .id();
    let mob_count_text = commands
        .spawn((
            MobCountText,
            TextBundle {
                text: Text::from_sections([
                    TextSection {
                        value: "Mobs: ".into(),
                        style: TextStyle {
                            font_size: 24.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 24.0,
                            color: Color::YELLOW,
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[mob_count_text]);
}

pub fn update_mob_counter(counter: Res<MobCount>, mut query: Query<&mut Text, With<MobCountText>>) {
    if let Ok(text) = &mut query.get_single_mut() {
        let value = counter.0;
        text.sections[1].value = format!("{value:>4.0}");
    }
}

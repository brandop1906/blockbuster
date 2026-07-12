use bevy::ui::prelude::*;
use bevy::prelude::*;

fn ui_setup(mut commands: Commands) {
  let container = Node {
    width: percent(15.0),
    height: percent(20.0),
    justify_content: JustifyContent::Center,
    ..default()
    };

  let square = (
    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
    Node {
      width: Val::Px(200.),
      border: UiRect::all(Val::Px(20.)),
      ..default()
    },
  );

  commands.spawn((container, children![(square)]));

  
}

fn spawn_text_in_ui(mut commands: Commands, assets: Res<AssetServer>) {
  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      bottom: px(680.0),
      right: px(1160.0),
      ..default()
    },
    Text::new("Score"),
    TextColor(Color::BLACK),
    TextLayout::new_with_justify(Justify::Center),
  ));
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (ui_setup, spawn_text_in_ui));
    }
}
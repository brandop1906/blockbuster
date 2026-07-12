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

fn spawn_text_in_ui(mut commands: Commands) {
  commands.spawn((
    Node {
      position_type: PositionType::Absolute,
      bottom: px(680.0),
      right: px(1160.0),
      ..default()
    },
    Text::new("Score: 0"),
    TextColor(Color::BLACK),
    TextLayout::new_with_justify(Justify::Center),
    ScoreText,
  ));
}

#[derive(Resource, Default)]
pub struct Score(pub u32);

 #[derive(Component)]
 struct ScoreText;

fn update_score_text(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if score.is_changed() {
        for mut text in &mut query {
            text.0 = format!("Score: {}", score.0);
        }
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, (ui_setup, spawn_text_in_ui))
            .add_systems(Update, update_score_text);
    }
}
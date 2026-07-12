use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};

use bevy_rapier2d::prelude::*;

use crate::ball::BallPlugin;
use crate::block::BlockPlugin;
use crate::paddle::*;
use crate::world::*;
use crate::ui::*;

mod paddle;
mod world;
mod ball;
mod ui;
mod block;

const BACKGROUND_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Fifo,
                    mode: WindowMode::Windowed,
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PaddlePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(BlockPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}



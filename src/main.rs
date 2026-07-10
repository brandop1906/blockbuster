use bevy::prelude::*;
use bevy::math::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};

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
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let rect_mesh = meshes.add(Rectangle::new(125.0, 25.0));

    let rect_material = materials.add(Color::srgb(0.2, 0.4, 0.8));

    commands.spawn((
        Mesh2d(rect_mesh),
        MeshMaterial2d(rect_material),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

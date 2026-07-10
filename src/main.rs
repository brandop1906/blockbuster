use bevy::prelude::*;
use bevy::math::prelude::*;
use bevy::time::Time;
use bevy::window::{PresentMode, Window, WindowMode};
use bevy::mesh::CircleMeshBuilder;
use bevy_rapier2d::prelude::*;

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
        .add_systems(Startup, (setup, setup_graphics))
        .add_systems(Update, (move_player, print_ball_altitude))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let rect_mesh = meshes.add(Rectangle::new(120.0, 24.0));

    let rect_material = materials.add(Color::srgb(0.2, 0.4, 0.8));

    commands.spawn((
        Mesh2d(rect_mesh),
        MeshMaterial2d(rect_material),
        RigidBody::Fixed,
        Collider::cuboid(60.0, 12.0),
        Transform::from_translation(Vec3::new(0.0, -250.0, 0.0)),
        Block,
    )).insert(GravityScale(0.0));

    let circle_mesh = CircleMeshBuilder::new(16.0, 64).build();

    let mesh_handle = meshes.add(circle_mesh);

    let circle_material = materials.add(Color::srgb(1.0, 0.0, 0.0));
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(circle_material),
        RigidBody::Dynamic,
        Collider::ball(16.0),
        Restitution::coefficient(0.7),
        Transform::from_xyz(0.0, 400.0, 0.0),
    ));
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

/*fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));
        
} */

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

#[derive(Component)]
pub struct Block;

fn move_player(
    mut q_movement: Query<&mut Transform, With<Block>>,
    input: ResMut<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut q_movement {
        if input.pressed(KeyCode::KeyD) { transform.translation.x += 400.0 * time.delta_secs(); }
        if input.pressed(KeyCode::KeyA) { transform.translation.x -= 400.0 * time.delta_secs(); }
    }
}

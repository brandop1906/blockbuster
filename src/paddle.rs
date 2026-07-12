use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Paddle;

fn paddle_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Paddle
    let rect_mesh = meshes.add(Rectangle::new(120.0, 24.0));

    let rect_material = materials.add(Color::srgb(0.2, 0.4, 0.8));

    commands.spawn((
        Mesh2d(rect_mesh),
        MeshMaterial2d(rect_material),
        RigidBody::Fixed,
        Collider::cuboid(60.0, 12.0),
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0),
        Transform::from_translation(Vec3::new(0.0, -250.0, 0.0)),
        Paddle,
    )).insert(GravityScale(0.0));
}

fn move_paddle(
    mut q_movement: Query<&mut Transform, With<Paddle>>,
    input: ResMut<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in &mut q_movement {
        if input.pressed(KeyCode::KeyD) { transform.translation.x += 400.0 * time.delta_secs(); }
        if input.pressed(KeyCode::KeyA) { transform.translation.x -= 400.0 * time.delta_secs(); }


        transform.translation.x = transform.translation.x.clamp(-330.0, 330.0);

    }
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,paddle_setup)
            .add_systems(Update,move_paddle);
    }
}
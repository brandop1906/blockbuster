use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::mesh::CircleMeshBuilder;

#[derive(Component)]
pub struct Ball;

fn ball_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    // Ball
    let circle_mesh = CircleMeshBuilder::new(16.0, 64).build();

    let mesh_handle = meshes.add(circle_mesh);

    let circle_material = materials.add(Color::srgb(1.0, 0.0, 0.0));
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(circle_material),
        RigidBody::Dynamic,
        Collider::ball(16.0),
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Transform::from_xyz(0.0, 100.0, 0.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        Velocity {
            linear: Vec2::new(25.0, -200.0),  // ← Add this!
            angular: 0.0,
        },
        ActiveEvents::COLLISION_EVENTS,
        Ball,
    )).insert(GravityScale(0.0));
}

fn maintain_ball_velocity(
    balls: Query<&mut Velocity, With<Ball>>,
) {
    for mut vel in balls {
        let speed = vel.linear.length();
        let min_speed = 300.0;

        if speed < min_speed && speed > 0.0 {
            vel.linear = vel.linear.normalize() * min_speed;
        }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,ball_setup)
            .add_systems(Update, maintain_ball_velocity);
    }
}
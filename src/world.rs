use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::ball::*;
use crate::paddle::*;
use bevy::mesh::CircleMeshBuilder;

#[derive(Component)]
pub struct BottomWall;

#[derive(Resource)]
struct CollisionCounter {
    spawn_cooldown: Timer,
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

fn wall_setup(
    mut commands: Commands,
) {
    // Left Wall
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 300.0),
        Transform::from_translation(Vec3::new(-400.0, 0.0, 0.0)),
        Restitution::coefficient(1.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        Friction::coefficient(0.0),
    ));

    // Right Wall
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 300.0),
        Transform::from_translation(Vec3::new(400.0, 0.0, 0.0)),
        Restitution::coefficient(1.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        Friction::coefficient(0.0),
    ));

    // Top Wall
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(500.0, 10.0),
        Transform::from_translation(Vec3::new(0.0, 300.0, 0.0)),
        Restitution::coefficient(1.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        Friction::coefficient(0.0),
    ));

    // Bottom Wall
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(500.0, 10.0),
        Transform::from_translation(Vec3::new(0.0, -400.0, 0.0)),
        Restitution::coefficient(1.0),
        Damping { linear_damping: 0.0, angular_damping: 0.0 },
        Friction::coefficient(0.0),
        BottomWall,
    ));
}

fn despawn_balls(
    mut collision_events: MessageReader<CollisionEvent>,
    mut commands: Commands,
    balls: Query<Entity, With<Ball>>,
    bottom_wall: Query<(), With<BottomWall>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                // Check if ball hit block
                let ball_hit_bottom_wall = 
                    (balls.contains(*e1) && bottom_wall.contains(*e2)) ||
                    (balls.contains(*e2) && bottom_wall.contains(*e1));

                if ball_hit_bottom_wall {
                    let ball_id = if balls.contains(*e1) { *e1 } else { *e2 };
                    commands.entity(ball_id).despawn();
                    }
                }
            CollisionEvent::Stopped(_, _, _) => {
                // Handle collision stop if needed
            }
        }
    }
}

fn detect_collisions(
    mut collision_events: MessageReader<CollisionEvent>,
    balls: Query<&mut Velocity, With<Ball>>,
    paddle: Query<&Transform, With<Paddle>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut counter: ResMut<CollisionCounter>,
    time: Res<Time>,
) {
    counter.spawn_cooldown.tick(time.delta());
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                // Check if ball hit block
                let ball_hit_block = 
                    (balls.contains(*e1) && paddle.contains(*e2)) ||
                    (balls.contains(*e2) && paddle.contains(*e1));

                if ball_hit_block && counter.spawn_cooldown.is_finished() {
                    if let Ok(paddle_transform) = paddle.single() {
                        counter.spawn_cooldown.reset();

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
                            Transform::from_xyz(
                                paddle_transform.translation.x, 
                                paddle_transform.translation.y + 30.0, 
                                0.0),
                            Damping { linear_damping: 0.0, angular_damping: 0.0 },
                            Velocity {
                                linear: Vec2::new(25.0, -200.0),  // ← Add this!
                                angular: 0.0,
                            },
                            ActiveEvents::COLLISION_EVENTS,
                            Ball,
                        )).insert(GravityScale(0.0));
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                // Handle collision stop if needed
            }
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CollisionCounter {spawn_cooldown: Timer::from_seconds(0.2, TimerMode::Once)})
            .add_systems(Startup, (setup_graphics, wall_setup))
            .add_systems(Update, (despawn_balls, detect_collisions));
    }
}
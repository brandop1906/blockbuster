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
        .add_systems(Update, (move_player, maintain_ball_velocity, detect_collisions, despawn_balls))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(CollisionCounter {spawn_cooldown: Timer::from_seconds(0.2, TimerMode::Once)})
        .run();
}

fn setup(
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

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2d::default());
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BottomWall;

fn move_player(
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

#[derive(Resource)]
struct CollisionCounter {
    spawn_cooldown: Timer,
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

fn despawn_balls(
    mut collision_events: MessageReader<CollisionEvent>,
    mut commands: Commands,
    balls: Query<Entity, With<Ball>>,
    bottom_wall: Query<(), With<BottomWall>>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                println!("TUCK IT");
                // Check if ball hit block
                let ball_hit_bottom_wall = 
                    (balls.contains(*e1) && bottom_wall.contains(*e2)) ||
                    (balls.contains(*e2) && bottom_wall.contains(*e1));

                if ball_hit_bottom_wall {
                    println!("WHYYYYYY");
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


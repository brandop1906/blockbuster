use bevy::{platform::collections::HashSet, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::{Rng, RngExt};

use crate::ball::Ball;
use crate::ui::Score;


const BLOCK_SIZE: Vec2 = Vec2::new(50.0, 20.0);
const SPAWN_MIN: Vec2 = Vec2::new(-200.0, -100.0);
const SPAWN_MAX: Vec2 = Vec2::new(200.0, 300.0);
const GAP: f32 = 5.0;      
const MAX_ATTEMPTS: usize = 20;

#[derive(Component)]
pub struct BlockSpawner {
    timer: Timer,
}

#[derive(Component)]
struct Block {
    cell: (i32, i32), // remember our cell so we can free it later
}

#[derive(Resource, Default)]
struct OccupiedCells(HashSet<(i32, i32)>);

fn spawn_blocks(
    mut commands: Commands,
    mut spawner_query: Query<&mut BlockSpawner>,
    mut occupied: ResMut<OccupiedCells>,
    time: Res<Time>,
) {
    let cell = BLOCK_SIZE + Vec2::splat(GAP);
    let cols = ((SPAWN_MAX.x - SPAWN_MIN.x) / cell.x).floor() as i32;
    let rows = ((SPAWN_MAX.y - SPAWN_MIN.y) / cell.y).floor() as i32;

    for mut spawner in spawner_query.iter_mut() {
        spawner.timer.tick(time.delta());
        if !spawner.timer.is_finished() {
            continue;
        }

        // Every cell that isn't already taken
        let free: Vec<(i32, i32)> = (0..cols)
            .flat_map(|i| (0..rows).map(move |j| (i, j)))
            .filter(|c| !occupied.0.contains(c))
            .collect();

        if free.is_empty() {
            continue; // grid is full
        }

        let mut rng = rand::rng();
        let (i, j) = free[rng.random_range(0..free.len())];
        occupied.0.insert((i, j));

        let pos = Vec2::new(
            SPAWN_MIN.x + cell.x * (i as f32 + 0.5),
            SPAWN_MIN.y + cell.y * (j as f32 + 0.5),
        );

        commands.spawn((
            Block { cell: (i, j) },
            Sprite {
                color: Color::srgb(1.0, 0.5, 0.5),
                custom_size: Some(BLOCK_SIZE),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
            RigidBody::Fixed,
            Collider::cuboid(BLOCK_SIZE.x / 2.0, BLOCK_SIZE.y / 2.0),
        ));
    }
}
    


fn despawn_blocks(
    mut collision_events: MessageReader<CollisionEvent>,
    mut commands: Commands,
    balls: Query<Entity, With<Ball>>,
    blocks: Query<&Block>,
    mut occupied: ResMut<OccupiedCells>,
    mut score: ResMut<Score>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                let ball_hit_block =
                    (balls.contains(*e1) && blocks.contains(*e2)) ||
                    (balls.contains(*e2) && blocks.contains(*e1));

                if ball_hit_block {
                    let block_id = if blocks.contains(*e1) { *e1 } else { *e2 };

                    if let Ok(block) = blocks.get(block_id) {
                        occupied.0.remove(&block.cell);
                    }
                    commands.entity(block_id).despawn();
                    score.0 += 1;
                }
            }
            CollisionEvent::Stopped(_, _, _) => {
                // Handle collision stop if needed
            }
        }
    }
}

fn setup_spawner(mut commands: Commands) {
    commands.spawn(BlockSpawner {
        timer: Timer::from_seconds(0.2, TimerMode::Repeating),
    });
}



pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OccupiedCells>()
            .add_systems(Startup, setup_spawner)
            .add_systems(Update, (spawn_blocks, despawn_blocks));
    }
}
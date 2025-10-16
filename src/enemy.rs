use bevy::prelude::*;

use crate::grid::{Grid, Lane};

#[derive(Component, Debug)]
#[require(Sprite, Transform, Lane)]
struct EnemyConfig {
    health: f32,
    speed: f32,
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: EnemyConfig,
    sprite: Sprite,
    transform: Transform,
    lane: Lane,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, enemy_setup);
        app.add_systems(FixedUpdate, enemy_system);
    }
}

fn enemy_setup(mut commands: Commands, grid: Res<Grid>) {
    for lane in 0..grid.height {
        commands.spawn(EnemyBundle {
            enemy: EnemyConfig {
                health: 100.0,
                speed: 10.0,
            },
            sprite: Sprite::default(),
            transform: Transform::from_xyz(100.0, ((100 * lane) - 500) as f32, 0.0), // idk where to put the zombies for now
            lane: Lane(lane),
        });
    }
}

fn enemy_system(mut enemy_query: Query<(&mut Transform, &EnemyConfig)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation.x -= enemy.speed * time.delta_secs();
        info!("{:?}", enemy);
    }
}

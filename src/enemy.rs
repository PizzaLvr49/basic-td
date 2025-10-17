use bevy::prelude::*;

use crate::grid::{Grid, Lane};

#[derive(Component, Debug)]
#[require(Sprite, Transform, Lane)]
struct Enemy {
    health: f32,
    speed: f32,
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
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
            enemy: Enemy {
                health: 100.0,
                speed: 10.0,
            },
            sprite: Sprite::from_color(Color::linear_rgb(1.0, 0.0, 0.0), Vec2::splat(100.0)),
            transform: Transform::from_xyz(500.0, 150.0 * lane as f32 - 375.0, 0.0), // TODO
            lane: Lane(lane),
        });
    }
}

fn enemy_system(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        transform.translation.x -= enemy.speed * time.delta_secs();
    }
}

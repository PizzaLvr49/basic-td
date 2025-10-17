use bevy::{math::bounding::Aabb2d, prelude::*};

use crate::grid::{Grid, GridX, Lane, TilePos};

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Tower {
    attack_speed: f32,
    damage: f32,
    position: TilePos,
    aabb: Aabb2d,
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct TowerProjectile {
    speed: f32,
    damage: f32,
    aabb: Aabb2d,
}

#[derive(Bundle)]
pub struct TowerBundle {
    tower_config: Tower,
    sprite: Sprite,
    transform: Transform,
    timer: TickTimer,
}

#[derive(Component)]
pub struct TickTimer(Timer);

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, tower_setup);
        app.add_systems(Update, tower_attack);
        app.add_systems(FixedUpdate, projectile_tick);
    }
}

fn tower_setup(mut commands: Commands, grid: Res<Grid>) {
    for lane in 0..grid.height {
        let grid_pos = TilePos {
            lane: Lane(lane),
            x: GridX(0),
        };

        let pos = Vec2::new(
            (grid_pos.x.0 as f32 * 150.0) - 500.0,
            150.0 * lane as f32 - 375.0,
        );

        commands.spawn(TowerBundle {
            tower_config: Tower {
                attack_speed: 1.0,
                damage: 10.0,
                position: grid_pos,
                aabb: Aabb2d::new(pos, Vec2::splat(50.0)),
            },
            sprite: Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::splat(100.0)),
            transform: Transform::from_translation(Vec3::from((pos, 0.0))),
            timer: TickTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
        });
    }
}

fn tower_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Tower, &mut TickTimer)>,
) {
    for (tower, mut ticker) in &mut query {
        if ticker.0.tick(time.delta()).just_finished() {
            let pos = Vec2::new(
                (tower.position.x.0 as f32 * 150.0) - 500.0,
                150.0 * tower.position.lane.0 as f32 - 375.0,
            );

            commands.spawn((
                TowerProjectile {
                    speed: 100.0,
                    damage: tower.damage,
                    aabb: Aabb2d::new(pos, Vec2::splat(20.0)),
                },
                Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::splat(40.0)),
                Transform::from_translation((pos, 0.0).into()),
            ));
        }
    }
}

fn projectile_tick() {}

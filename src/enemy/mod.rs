use self::formation::{Formation, FormationMaker};
use crate::components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity};
use crate::{
    EnemyCount, GameTextures, WinSize, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE, SPRITE_SCALE,
};

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::{thread_rng, Rng};
use std::{f32::consts::PI, time::Duration};

mod formation;

/// 敌人插件
/// 负责敌人生成、开火、以及沿椭圆轨迹移动的全部逻辑
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // 插入敌人阵型生成器资源
            .insert_resource(FormationMaker::default())
            // 每隔 1 秒尝试生成一个敌人（受 ENEMY_MAX 限制）
            .add_systems(
                Update,
                enemy_spawn_system.run_if(on_timer(Duration::from_secs(1))),
            )
            // 敌人开火系统（使用随机条件触发）
            .add_systems(Update, enemy_fire_system.run_if(enemy_fire_criteria))
            // 敌人移动系统（沿椭圆轨迹飞行）
            .add_systems(Update, enemy_movement_system);
    }
}

/// 敌人生成系统
/// 每秒执行一次：如果当前敌人数量未达到上限，则生成一个新敌人
fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,      // 当前场上敌人数量
    mut formation_maker: ResMut<FormationMaker>, // 敌人阵型生成器
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < ENEMY_MAX {
        // 获取一个新的阵型，并得到起始坐标
        let formation = formation_maker.make(&win_size);
        let (x, y) = formation.start;
        commands
            .spawn((
                Sprite::from_image(game_textures.enemy.clone()), // 使用敌人纹理
                Transform {
					// 2D 游戏里 Vec3(x, y, z) 的 z 不是位置深度，而是“图层顺序”。
                    // z 值越大 → 越靠前（越在上方）。
                    // Z=10 在这里只是一个随意选择的数字，只要比激光（Z=0）大，就能“在激光上方”。
                    translation: Vec3::new(x, y, 10.),           // 生成位置（Z=10 在玩家上方）
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
            ))
            .insert(Enemy)                    // 标记为敌人
            .insert(formation)                // 插入阵型组件（包含移动轨迹参数）
            .insert(SpriteSize::from(ENEMY_SIZE)); // 记录精灵尺寸（用于碰撞检测）

        // 敌人数量 +1
        enemy_count.0 += 1;
    }
}

/// 敌人开火触发条件
/// 每帧有大约 1/60 的概率返回 true（即平均每秒开火约 1 次）
fn enemy_fire_criteria() -> bool {
    thread_rng().gen_bool(1. / 60.)
}

/// 敌人开火系统
/// 对每个敌人生成一发向下射击的激光
fn enemy_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,   // 查询所有敌人的位置
) {
    for &tf in enemy_query.iter() {
        let (x, y) = (tf.translation.x, tf.translation.y);
        // 生成敌方激光
        commands
            .spawn((
                Sprite::from_image(game_textures.enemy_laser.clone()),
                Transform {
                    translation: Vec3::new(x, y - 15., 0.),      // 在敌人下方一点生成
                    rotation: Quat::from_rotation_x(PI),        // 旋转 180 度，让激光向下
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                },
            ))
            .insert(Laser)                    // 标记为激光
            .insert(SpriteSize::from(ENEMY_LASER_SIZE))   // 激光尺寸
            .insert(FromEnemy)                // 标记为敌人发射的激光
            .insert(Movable { auto_despawn: true })       // 移出屏幕后自动销毁
            .insert(Velocity { x: 0., y: -1. });          // 向下移动
    }
}

/// 敌人移动系统
/// 让每个敌人沿着一个椭圆轨迹平滑移动
fn enemy_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>,
) {
    let delta = time.delta_secs();   // 每帧时间间隔（秒）
    for (mut transform, mut formation) in &mut query {
        // 当前敌人位置
        let (x_org, y_org) = (transform.translation.x, transform.translation.y);
        // 本帧允许移动的最大距离
        let max_distance = delta * formation.speed;
        // 旋转方向：1 为逆时针，-1 为顺时针（根据起始位置决定）
        let dir: f32 = if formation.start.0 < 0. { 1. } else { -1. };
        let (x_pivot, y_pivot) = formation.pivot;   // 椭圆中心点
        let (x_radius, y_radius) = formation.radius; // 椭圆半径（x、y 方向）

        // 计算下一帧的角度（基于时间和速度）
        let angle = formation.angle
            + dir * formation.speed * delta / (x_radius.min(y_radius) * PI / 2.);

        // 计算椭圆轨迹上的目标点坐标
        let x_dst = x_radius * angle.cos() + x_pivot;
        let y_dst = y_radius * angle.sin() + y_pivot;

        // 计算当前点到目标点的距离
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();

        // 计算本帧应该移动的比例（防止一次跳过太多）
        let distance_ratio = if distance == 0. { 0. } else { max_distance / distance };

        // 计算最终移动后的坐标（带插值，防止穿透）
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. { x.max(x_dst) } else { x.min(x_dst) }; // 限制不超过目标点
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. { y.max(y_dst) } else { y.min(y_dst) };

        // 只有当敌人已经接近或到达椭圆轨迹上时，才更新角度（防止角度提前变化导致轨迹错误）
        if distance < max_distance * formation.speed / 20. {
            formation.angle = angle;
        }

        // 更新敌人位置
        let translation = &mut transform.translation;
        (translation.x, translation.y) = (x, y);
    }
}
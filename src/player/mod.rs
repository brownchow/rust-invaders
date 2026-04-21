use crate::components::{FromPlayer, Laser, Movable, Player, SpriteSize, Velocity};
use crate::{
    GameTextures, PlayerState, WinSize, PLAYER_LASER_SIZE, PLAYER_RESPAWN_DELAY, PLAYER_SIZE,
    SPRITE_SCALE,
};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

/// 玩家插件
/// 负责管理玩家相关的所有系统和资源
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // 插入玩家状态资源（记录是否存活、上次死亡时间等）
            .insert_resource(PlayerState::default())
            // 每 0.5 秒尝试执行一次玩家生成系统（用于复活逻辑）
            .add_systems(
                Update,
                player_spawn_system.run_if(on_timer(Duration::from_secs_f32(0.5))),
            )
            // 玩家键盘输入处理系统（控制左右移动）
            .add_systems(Update, player_keyboard_event_system)
            // 玩家开火系统（空格键发射激光）
            .add_systems(Update, player_fire_system);
    }
}

/// 玩家生成系统（复活系统）
/// 每隔 0.5 秒检查一次：如果玩家当前不在场上，且已经过了复活延迟时间，则生成玩家
fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,   // 玩家状态（可修改）
    time: Res<Time>,                         // 当前游戏时间
    game_textures: Res<GameTextures>,        // 游戏纹理资源（玩家飞船图片）
    win_size: Res<WinSize>,                  // 窗口尺寸
) {
    let now = time.elapsed_secs_f64();           // 当前时间（秒）
    let last_shot = player_state.last_shot;      // 上次玩家死亡/被击毁的时间
    // 条件：玩家当前不在场上，并且已经过了复活延迟时间（或首次生成）
    if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
        // 计算玩家生成的 Y 坐标（屏幕最下方上方一点）
        let bottom = -win_size.h / 2.;
        commands
            .spawn((
                // 使用玩家飞船纹理创建 Sprite
                Sprite::from_image(game_textures.player.clone()),
                // 设置初始位置和缩放
                Transform {
                    translation: Vec3::new(
                        0.,                                           // X 坐标居中
                        bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., // Y 坐标：屏幕底部 + 一定偏移
                        10.,                                          // Z 坐标（渲染层级）
                    ),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),   // 缩放比例
                    ..Default::default()
                },
            ))
            // 插入各种组件
            .insert(Player)                                      // 标记为玩家
            .insert(SpriteSize::from(PLAYER_SIZE))               // 记录精灵尺寸（用于碰撞）
            .insert(Movable { auto_despawn: false })             // 可移动，但不自动销毁
            .insert(Velocity { x: 0., y: 0. });                  // 初始速度为 0
        // 更新玩家状态：标记为已生成
        player_state.spawned();
    }
}

/// 玩家开火系统
/// 检测空格键按下，生成两发激光（左右对称）
fn player_fire_system(
    mut commands: Commands,
    kb: Res<ButtonInput<KeyCode>>,           // 键盘输入
    game_textures: Res<GameTextures>,        // 激光纹理
    query: Query<&Transform, With<Player>>,  // 获取玩家位置
) {
    // 获取玩家 Transform（如果存在）
    if let Ok(player_tf) = query.get_single() {
        // 检测空格键刚刚被按下（防止连发）
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            // 计算激光生成时的左右偏移量
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;
            // 定义一个闭包，用于生成单发激光
            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn((
                        Sprite::from_image(game_textures.player_laser.clone()),
                        Transform {
                            translation: Vec3::new(x + x_offset, y + 15., 0.), // 激光生成在玩家上方一点
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..Default::default()
                        },
                    ))
                    .insert(Laser)                    // 标记为激光
                    .insert(FromPlayer)               // 标记为玩家发射的激光
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))  // 激光尺寸
                    .insert(Movable { auto_despawn: true })       // 移出屏幕后自动销毁
                    .insert(Velocity { x: 0., y: 1. });           // 向上移动（速度为1）
            };
            // 生成左右两发激光
            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}

/// 玩家键盘移动系统
/// 根据左右箭头键实时更新玩家的水平速度
fn player_keyboard_event_system(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,   // 只查询玩家的 Velocity 组件
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        // 按左箭头 → 向左移动
        // 按右箭头 → 向右移动
        // 都不按   → 速度归零
        velocity.x = if kb.pressed(KeyCode::ArrowLeft) {
            -1.
        } else if kb.pressed(KeyCode::ArrowRight) {
            1.
        } else {
            0.
        };
    }
}
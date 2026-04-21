use bevy::math::{Vec2, Vec3};
use bevy::prelude::Component;
use bevy::time::{Timer, TimerMode};

// ======================== 通用组件 ========================

/// 速度组件
/// 用于表示实体在 x 和 y 方向上的移动速度
#[derive(Component)]
pub struct Velocity {
    pub x: f32,   // X 轴速度（正数向右，负数向左）
    pub y: f32,   // Y 轴速度（正数向上，负数向下）
}

/// 可移动组件
/// 标记实体是否可以移动，并控制是否自动销毁
#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,   // true 表示当实体移出屏幕后自动销毁
}

// ======================== 激光相关组件 ========================

/// 激光标记组件
/// 只要实体带有此组件，就表示它是一颗激光（玩家激光或敌方激光）
#[derive(Component)]
pub struct Laser;

/// 精灵尺寸组件
/// 用于记录精灵的宽和高，方便碰撞检测和缩放
#[derive(Component)]
pub struct SpriteSize(pub Vec2);

/// 方便从元组 (f32, f32) 转换为 SpriteSize
impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

// ======================== 玩家相关组件 ========================

/// 玩家标记组件
/// 标记该实体是玩家飞船
#[derive(Component)]
pub struct Player;

/// 来自玩家的标记
/// 用于区分激光是由玩家发射的（便于碰撞检测逻辑）
#[derive(Component)]
pub struct FromPlayer;

// ======================== 敌人相关组件 ========================

/// 敌人标记组件
/// 标记该实体是敌方飞船
#[derive(Component)]
pub struct Enemy;

/// 来自敌人的标记
/// 用于区分激光是由敌人发射的
#[derive(Component)]
pub struct FromEnemy;

// ======================== 爆炸相关组件 ========================

/// 爆炸标记组件
/// 标记该实体是一个爆炸特效
#[derive(Component)]
pub struct Explosion;

/// 需要生成爆炸的组件
/// 当需要创建爆炸时，可以向实体插入此组件，系统会根据位置生成爆炸
#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);   // Vec3 表示爆炸在世界坐标中的生成位置

/// 爆炸动画定时器组件
/// 控制爆炸动画每帧切换的速度
#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

/// 为 ExplosionTimer 提供默认值
/// 默认每 0.05 秒切换一次动画帧（即每秒切换 20 帧）
impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}
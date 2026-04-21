#![allow(unused)] // silence unused warnings while exploring (to comment out)

use bevy::math::bounding::IntersectsVolume;
use bevy::math::{bounding::Aabb2d, Vec3Swizzles};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use components::{
	Enemy, Explosion, ExplosionTimer, ExplosionToSpawn, FromEnemy, FromPlayer, Laser, Movable,
	Player, SpriteSize, Velocity,
};
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use std::collections::HashSet;

mod components;
mod enemy;
mod player;

// 玩家背景图、玩家大小、玩家激光、玩家激光大小
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

// 敌人背景图、敌人大小、敌人激光、敌人激光大小
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);

// 爆炸背景图、爆炸长度
const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const EXPLOSION_LEN: usize = 16;

// 精灵缩放比例
const SPRITE_SCALE: f32 = 0.5;
const BASE_SPEED: f32 = 500.;
// 玩家死了后，重新出现的时间间隔，单位：秒
const PLAYER_RESPAWN_DELAY: f64 = 2.;
// 最大敌人数
const ENEMY_MAX: u32 = 2;
// 编队最大成员数
const FORMATION_MEMBERS_MAX: u32 = 2;

// 当前屏幕大小
#[derive(Resource)]
pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

/// 游戏纹理资源，存储所有游戏图像
#[derive(Resource)]
struct GameTextures {
	// 玩家纹理
	player: Handle<Image>,
	// 玩家激光纹理
	player_laser: Handle<Image>,
	// 敌人纹理
	enemy: Handle<Image>,
	// 敌人激光纹理
	enemy_laser: Handle<Image>,
	// 爆炸布局纹理
	explosion_layout: Handle<TextureAtlasLayout>,
	// 爆炸纹理
	explosion_texture: Handle<Image>,
}

// 敌人个数
#[derive(Resource)]
struct EnemyCount(u32);

#[derive(Resource)]
struct PlayerState {
	/// 是否存活，false 死亡，true 存活
	on: bool,    
	/// 记录玩家被击中的时间，-1 表示没有被击中
	last_shot: f64,
}

// 类似 java 里的构造函数，初始化玩家状态为死亡状态，记录玩家被击中的时间为-1
impl Default for PlayerState {
	fn default() -> Self {
		Self {
			on: false,
			last_shot: -1.,
		}
	}
}

impl PlayerState {
	// 玩家被击中时，调用此方法
	pub fn shot(&mut self, time: f64) {
		self.on = false;
		self.last_shot = time;
	}
	// 玩家重新出现时，调用此方法
	/// 重置玩家状态，将玩家设置为存活状态，记录玩家被击中的时间为-1
	pub fn spawned(&mut self) {
		self.on = true;
		self.last_shot = -1.;
	}
}

// 游戏主函数
fn main() {
	App::new()
		.insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				title: "Rust Invaders!".into(),
				resolution: (598., 676.).into(),
				// position window (for tutorial)
				// position: WindowPosition::At(IVec2::new(2780, 4900)),
				..Default::default()
			}),
			..Default::default()
		}))
		.add_plugins(PlayerPlugin)
		.add_plugins(EnemyPlugin)
		.add_systems(Startup, setup_system)
		.add_systems(Update, movable_system)
		.add_systems(Update, player_laser_hit_enemy_system)
		.add_systems(Update, enemy_laser_hit_player_system)
		.add_systems(Update, explosion_to_spawn_system)
		.add_systems(Update, explosion_animation_system)
		.run();
}

fn setup_system(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
	query: Query<&Window, With<PrimaryWindow>>,
) {
	// 在游戏世界中创建一个 2D 相机实体，用于2d游戏
	commands.spawn(Camera2d);
	// 获取窗口大小
	let primary = match query.get_single() {
		Ok(window) => window,
		Err(_) => return,
	};
	let (win_w, win_h) = (primary.width(), primary.height());
	// add WinSize resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);
	// 创建爆炸效果的纹理图集  atlas 地图集
	let texture_handle = asset_server.load(EXPLOSION_SHEET);
	// 定义 4x4 网格
	let texture_atlas = TextureAtlasLayout::from_grid(        
		UVec2::new(64, 64),  // 每个精灵大小 64x64
		4, 4,                // 4 行 4 列
		None, None
	);
	// 添加到资源
	let explosion_layout = texture_atlases.add(texture_atlas);
	// 创建游戏纹理资源
	let game_textures = GameTextures {
		player: asset_server.load(PLAYER_SPRITE),
		player_laser: asset_server.load(PLAYER_LASER_SPRITE),
		enemy: asset_server.load(ENEMY_SPRITE),
		enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
		explosion_layout,
		explosion_texture: texture_handle,
	};
	// command 添加资源
	commands.insert_resource(game_textures);
	commands.insert_resource(EnemyCount(0));
}

// 移动系统，更新游戏世界中的实体位置
fn movable_system(
	mut commands: Commands,
	time: Res<Time>,
	win_size: Res<WinSize>,
	mut query: Query<(Entity, &Velocity, &mut Transform, &Movable), Without<Player>>,
	mut player_query: Query<(&Velocity, &mut Transform, &SpriteSize), With<Player>>,
) {
	let delta = time.delta_secs();
	for (entity, velocity, mut transform, movable) in &mut query {
		let translation = &mut transform.translation;
		translation.x += velocity.x * delta * BASE_SPEED;
		translation.y += velocity.y * delta * BASE_SPEED;
		if movable.auto_despawn {
			const MARGIN: f32 = 200.;
			// 坐标系统：原点 (0,0) 在屏幕中心，x轴向右为正，y轴向上为正
			if translation.y > win_size.h / 2. + MARGIN
				|| translation.y < -win_size.h / 2. - MARGIN
				|| translation.x > win_size.w / 2. + MARGIN
				|| translation.x < -win_size.w / 2. - MARGIN
			{
				commands.entity(entity).despawn();
			}
		}
	}
	// 使用 clamp 函数将玩家位置限制在 [-win_width/2 + half_width, win_width/2 - half_width] 范围内
	for (velocity, mut transform, sprite_size) in &mut player_query {
		let scale_x = transform.scale.x;
		let translation = &mut transform.translation;
		translation.x += velocity.x * delta * BASE_SPEED;
		translation.y += velocity.y * delta * BASE_SPEED;

		let half_width = sprite_size.0.x * scale_x / 2.;
		let min_x = -win_size.w / 2. + half_width;
		let max_x = win_size.w / 2. - half_width;
		translation.x = translation.x.clamp(min_x, max_x);
	}

}

// 玩家击中敌人
#[allow(clippy::type_complexity)] // for the Query types.
fn player_laser_hit_enemy_system(
	mut commands: Commands,
	mut enemy_count: ResMut<EnemyCount>,
	laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
	enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
) {
	// 已销毁的实体
	let mut despawned_entities: HashSet<Entity> = HashSet::new();
	// 遍历激光
	for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
		if despawned_entities.contains(&laser_entity) {
			continue;
		}
		let laser_scale = laser_tf.scale.xy();
		// 遍历敌人
		for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
			if despawned_entities.contains(&enemy_entity)
				|| despawned_entities.contains(&laser_entity)
			{
				continue;
			}
			let enemy_scale = enemy_tf.scale.xy();
			// 判断是否碰撞
			let laser_aabb = Aabb2d::new(
				laser_tf.translation.truncate(),
				(laser_size.0 * laser_scale) / 2.,
			);
			let enemy_aabb = Aabb2d::new(
				enemy_tf.translation.truncate(),
				(enemy_size.0 * enemy_scale) / 2.,
			);
			let collision = laser_aabb.intersects(&enemy_aabb);
			// 执行碰撞逻辑
			if collision {
				// 销毁敌人
				commands.entity(enemy_entity).despawn();
				despawned_entities.insert(enemy_entity);
				enemy_count.0 -= 1;				
				// 销毁激光
				commands.entity(laser_entity).despawn();
				despawned_entities.insert(laser_entity);
				// 生成爆炸效果
				commands.spawn(ExplosionToSpawn(enemy_tf.translation));
			}
		}
	}
}

// 敌人击中玩家
#[allow(clippy::type_complexity)] // for the Query types.
fn enemy_laser_hit_player_system(
	mut commands: Commands,
	mut player_state: ResMut<PlayerState>,
	time: Res<Time>,
	laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
	player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
) {
	if let Ok((player_entity, player_tf, player_size)) = player_query.get_single() {
		let player_scale = player_tf.scale.xy();
		// 遍历激光
		// 所有已销毁的实体集合
		let mut despawned_entities: HashSet<Entity> = HashSet::new();
		for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
			let laser_scale = laser_tf.scale.xy();
			// 判断是否碰撞
			let collision = Aabb2d::new(
				laser_tf.translation.truncate(),
				(laser_size.0 * laser_scale) / 2.,
			)
			.intersects(&Aabb2d::new(
				player_tf.translation.truncate(),
				(player_size.0 * player_scale) / 2.,
			));
			// 执行碰撞逻辑
			if collision {
				// 销毁玩家
				commands.entity(player_entity).despawn();
				player_state.shot(time.elapsed_secs_f64());
				// 销毁激光
				commands.entity(laser_entity).despawn();
				// 生成爆炸效果
				commands.spawn(ExplosionToSpawn(player_tf.translation));
				break;
			}
		}
	}
}

// 生成爆炸效果
fn explosion_to_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	query: Query<(Entity, &ExplosionToSpawn)>,
) {
	for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
		// 生成爆炸精灵图 / 实例化爆炸特效贴图 
		commands
			.spawn((
				Sprite {
					image: game_textures.explosion_texture.clone(),
					texture_atlas: Some(TextureAtlas {
						layout: game_textures.explosion_layout.clone(),
						index: 0,
					}),
					..Default::default()
				},
				Transform::from_translation(explosion_to_spawn.0),
			))
			.insert(Explosion)
			.insert(ExplosionTimer::default());
		// 销毁/移除即将生成的爆炸
		commands.entity(explosion_spawn_entity).despawn();
	}
}

// 爆炸动画效果系统
// 作用：每帧更新爆炸实体的动画帧，当动画播放完毕后自动销毁实体
fn explosion_animation_system(
    mut commands: Commands,           // 用于对实体进行增删改操作的命令队列
    time: Res<Time>,                  // 全局时间资源，用于获取帧间隔时间（delta time）
    mut query: Query<                 // 查询所有带有 Explosion 标记的实体
        (
            Entity,                   // 实体 ID
            &mut ExplosionTimer,      // 爆炸定时器（控制动画帧切换速度）
            &mut Sprite,              // 精灵组件，用于控制纹理图集（动画帧）
        ),
        With<Explosion>,              // 只查询带有 Explosion 组件的实体
    >,
) {
    // 遍历所有正在播放爆炸动画的实体
    for (entity, mut timer, mut sprite) in &mut query {
        // 1. 更新定时器，让它随着游戏时间流逝
        //    timer.0 通常是一个 Timer 组件，用于控制帧切换间隔
        timer.0.tick(time.delta());
        // 2. 判断定时器是否完成一次计时（即到了该切换下一帧的时间）
        if timer.0.finished() {
            // 3. 获取当前实体的纹理图集（TextureAtlas），用于切换动画帧
            if let Some(texture) = sprite.texture_atlas.as_mut() {
                // 切换到下一帧
                texture.index += 1;
                // 4. 如果已经播放到最后一帧（超过爆炸动画总帧数）
                if texture.index >= EXPLOSION_LEN {
                    // 则销毁这个爆炸实体，释放资源
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

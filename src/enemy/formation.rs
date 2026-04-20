use crate::{WinSize, BASE_SPEED, FORMATION_MEMBERS_MAX};
use bevy::prelude::{Component, Resource};
use rand::{thread_rng, Rng};

/// 组件 - 敌人编队（每个敌人共享一组编队参数）
///
/// #[derive(...)] 是 Rust 的派生宏语法，相当于 Java 的注解处理器自动生成代码：
/// - Clone: 类似于 Java implements Cloneable 接口，允许使用 .clone() 复制实例  
/// - Component: 类似于 Java extends ECS 框架的 Component 基类，标记为 Bevy ECS 的组件
#[derive(Clone, Component)]
pub struct Formation {
	/**
	 * 编队的初始位置，敌人从这里出现进入屏幕
	 */
	pub start: (f32, f32),

	/**
	 * 编队绕枢轴点运动时的椭圆半径（水平和垂直方向）
	 */
	pub radius: (f32, f32),

	/**
	 * 编队的枢轴点，敌人绕此点做椭圆运动
	 */
	pub pivot: (f32, f32),

	/**
	 * 编队的移动速度
	 */
	pub speed: f32,

	/**
	 * 编队当前的运动角度，每帧更新
	 */
	pub angle: f32,
}

/// 资源 - 编队生成器，用于按模板批量创建敌人编队参数
#[derive(Default, Resource)]
pub struct FormationMaker {
	/// 当前正在复用的编队模板
	current_template: Option<Formation>,

	/// 当前模板已创建的成员数量
	current_members: u32,
}

/// 编队生成器实现
impl FormationMaker {

	/// 创建新的编队
	///
	/// 如果当前模板为 None，或当前模板已满，则创建新编队。
	/// 如果当前模板存在，则复用模板创建新编队。
	///
	/// 返回新的编队实例。
	pub fn make(&mut self, win_size: &WinSize) -> Formation {
		match (&self.current_template, self.current_members >= FORMATION_MEMBERS_MAX) {
			// 有当前模板且未达到最大成员数
			(Some(tmpl), false) => {
				// tmpl 就是 Formation 的值
				// // Java: Optional 的类似写法
				// if (template.isPresent()) {
				//     Formation tmpl = template.get();
				//     // 使用 tmpl
				// }
				self.current_members += 1;
				tmpl.clone()
			}
			// 第一次创建或前一个模板已满（需要创建新模板）
			(None, _) | (_, true) => {
				let mut rng = thread_rng();
				// 计算初始位置
				// 坐标系：原点(0,0)在屏幕中心，X轴向右为正，Y轴向上为正
				// 敌人从屏幕左右两侧边缘外100像素处出现
				let w_span = win_size.w / 2. + 100.;
				let h_span = win_size.h / 2. + 100.;
				let x = if rng.gen_bool(0.5) { w_span } else { -w_span };  // 50%概率右侧，50%概率左侧
				let y = rng.gen_range(-h_span..h_span);  // Y轴在屏幕范围内随机
				let start = (x, y);

				// 计算枢轴点
				let w_span = win_size.w / 4.;
				let h_span = win_size.h / 3. - 50.;
				let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));

				// 计算运动半径
				let radius = (rng.gen_range(80.0..150.), 100.);

				// 计算初始角度
				let angle = (y - pivot.1).atan2(x - pivot.0);

				// 移动速度（暂时固定）
				let speed = BASE_SPEED;

				// 创建编队
				let formation = Formation {
					start,
					radius,
					pivot,
					speed,
					angle,
				};

				// 保存为当前模板
				self.current_template = Some(formation.clone());
				// 重置成员计数为 1
				self.current_members = 1;

				formation
			}
		}
	}
}

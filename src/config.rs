use crate::frog::{new_zone, Zone};

// 环境相关配置
pub const ENV_EGG_QTY: usize = 80;
pub const ENV_FOOD_QTY: usize = 2000;
pub const ENV_X_SIZE: usize = 300;
pub const ENV_Y_SIZE: usize = 300;
pub const ENV_STEPS_PER_ROUND: usize = 3000;

// 青蛙相关配置
pub const FROG_INIT_ENERGY: f32 = 1000f32;
pub const FROG_BRAIN_RADIUS: f32 = 1000f32;

// 蝌蚪相关配置
pub const EGG_CELL_GROUP_QTY: usize = 300;
pub const EGG_BRAIN_RADIUS: usize = 1000;

// 食物相关配置
pub const FOOD_ENERGY: f32 = 1000f32;

// 运动神经区域
pub const ZONE_MOVE_UP: Zone = new_zone(500f32, 50f32, 10f32);
pub const ZONE_MOVE_DOWN: Zone = new_zone(500f32, 100f32, 10f32);
pub const ZONE_MOVE_LEFT: Zone = new_zone(500f32, 150f32, 10f32);
pub const ZONE_MOVE_RIGHT: Zone = new_zone(500f32, 200f32, 10f32);
pub const ZONE_MOVE_RANDOM: Zone = new_zone(500f32, 300f32, 10f32);

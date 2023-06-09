use rand::Rng;

use crate::{
    config::{
        EGG_BRAIN_RADIUS, ENV_X_SIZE, ENV_Y_SIZE, FOOD_ENERGY, FROG_INIT_ENERGY, ZONE_MOVE_DOWN,
        ZONE_MOVE_LEFT, ZONE_MOVE_RANDOM, ZONE_MOVE_RIGHT, ZONE_MOVE_UP,
    },
    egg::Egg,
    env::Env,
};

pub struct Frog {
    x: usize,
    y: usize,
    egg: Egg,
    cells: Vec<Cell>, // 神经元数量
    energy: f32,      // 能量
    alive: bool,
    move_count: i32,
    brain_radius: f32,
    eat_foods: i32,
}

impl Frog {
    pub fn new(x: usize, y: usize, egg: Egg) -> Frog {
        let mut cells = Vec::new();
        for cell_group in egg.get_cell_groups().iter() {
            let cell = Cell::new(cell_group);
            cells.push(cell);
        }
        let brain_radius = egg.get_brain_radius();
        Frog {
            x,
            y,
            egg,
            cells,
            energy: FROG_INIT_ENERGY,
            alive: true,
            move_count: 0,
            brain_radius,
            eat_foods: 0,
        }
    }
    pub fn active(&mut self, env: &mut Env) -> bool {
        // 已经死亡，直接结束
        if !self.alive {
            return false;
        }
        // 越界者死
        if self.x < 0 || self.x >= ENV_X_SIZE || self.y < 0 || self.y >= ENV_Y_SIZE {
            return false;
        }
        let cells = &self.cells;
        for cell in cells {
            for output in &cell.outputs {
                if ZONE_MOVE_UP.nearby_output(output) {
                    self.y += 1;
                    if self.y >= ENV_Y_SIZE {
                        self.alive = false;
                        continue;
                    }
                    self.check_food_and_eat(env)
                    // self.move_up(env);
                } else if ZONE_MOVE_DOWN.nearby_output(output) {
                    self.move_down(env);
                } else if ZONE_MOVE_LEFT.nearby_output(output) {
                    self.move_left(env);
                } else if ZONE_MOVE_RIGHT.nearby_output(output) {
                    self.move_right(env);
                } else if ZONE_MOVE_RANDOM.nearby_output(output) {
                    self.move_random(env);
                }
            }
        }
        true
    }
    fn move_up(&mut self, env: &mut Env) {
        self.y += 1;
        if self.y >= ENV_Y_SIZE {
            self.alive = false;
            return;
        }
        self.check_food_and_eat(env)
    }
    fn move_down(&mut self, env: &mut Env) {
        self.y -= 1;
        if self.y < 0 {
            self.alive = false;
            return;
        }
        self.check_food_and_eat(env)
    }
    fn move_left(&mut self, env: &mut Env) {
        self.x -= 1;
        if self.x < 0 {
            self.alive = false;
            return;
        }
        self.check_food_and_eat(env)
    }
    fn move_right(&mut self, env: &mut Env) {
        self.x += 1;
        if self.x >= ENV_Y_SIZE {
            self.alive = false;
            return;
        }
        self.check_food_and_eat(env)
    }
    fn move_random(&mut self, env: &mut Env) {
        let value = rand::thread_rng().gen_range(1..4);
        if value == 1 {
            self.move_left(env);
        } else if value == 2 {
            self.move_right(env);
        } else if value == 3 {
            self.move_up(env);
        } else if value == 4 {
            self.move_down(env);
        }
    }

    fn check_food_and_eat(&mut self, env: &mut Env) {
        let mut eat_food = false;
        if env.exist_food(self.x, self.y) {
            env.delete_food(self.x, self.y);
            self.energy += FOOD_ENERGY;
            self.eat_foods += 1;
            // TODO 奖励
        }
    }
}

/// 神经元组：代表了一束相同功能和结构、分布位置相近的脑神经元.
///
/// 目的是为了下蛋时简化串行化海量的神经元, 只需要在egg里定义一组cellGroup就行了，
/// 不需要将海量的一个个的神经元串行化存放到egg里，这样一来Frog就不能"永生"了，
/// 因为每一个egg都不等同于它的母体，
/// 而且每一次测试，一些复杂的条件反射的建立都必须从头开始训练，
/// 在项目后期，有可能每个frog生命的一半时间都花在重新建立条件反射的学习过程中。
///
/// 模拟一公一母两个蛋受精，CellGroup叠加也许很fun,这样可以将不同环境训练出的蛋叠加成一个。但现在暂时不考虑。
#[derive(Clone)]
pub struct CellGroup {
    group_input_zone: Zone,
    group_output_zone: Zone,
    cell_input_radius: f32,
    cell_qty: i32, // quantity 数量
    cell_output_radius: f32,
    input_qty_per_cell: i32,
    output_qty_per_cell: i32,
}

impl CellGroup {
    pub fn new() -> Self {
        use rand::Rng;
        let group_input_zone = Zone::new(EGG_BRAIN_RADIUS as f32);
        let group_output_zone = Zone::new(EGG_BRAIN_RADIUS as f32);
        CellGroup {
            group_input_zone,
            group_output_zone,
            cell_input_radius: rand::random::<f32>() * 0.001,
            cell_output_radius: rand::random::<f32>() * 0.001,
            cell_qty: rand::thread_rng().gen_range(1..10),
            input_qty_per_cell: rand::thread_rng().gen_range(1..10),
            output_qty_per_cell: rand::thread_rng().gen_range(1..5),
        }
    }
}

/// 神经元
pub struct Cell {
    inputs: Vec<InputZone>,
    outputs: Vec<OutputZone>,
}

impl Cell {
    pub fn new(cell_group: &CellGroup) -> Self {
        let mut inputs: Vec<InputZone> = Vec::new();
        for _ in 0..cell_group.input_qty_per_cell {
            let z = &cell_group.group_input_zone;
            let x: f32 = z.x - z.radius + z.radius * 2f32 * rand::random::<f32>();
            let z = &cell_group.group_output_zone;
            let y: f32 = z.x - z.radius + z.radius * 2f32 * rand::random::<f32>();
            let input = InputZone::new(x, y, cell_group.cell_input_radius);
            inputs.push(input)
        }
        let mut outputs: Vec<OutputZone> = Vec::new();
        for _ in 0..cell_group.output_qty_per_cell {
            let z = &cell_group.group_output_zone;
            let x: f32 = z.x - z.radius + z.radius * 2f32 * rand::random::<f32>();
            let y: f32 = z.x - z.radius + z.radius * 2f32 * rand::random::<f32>();
            let output = OutputZone::new(x, y, cell_group.cell_input_radius);
            outputs.push(output)
        }
        Cell { inputs, outputs }
    }
}

/// 神经元输入区域
#[derive(Clone)]
pub struct InputZone {
    x: f32,
    y: f32,
    radius: f32,
    energy: f32,
}

impl InputZone {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        InputZone {
            x,
            y,
            radius,
            energy: 1000f32,
        }
    }
}

/// 神经元输出区域
#[derive(Clone)]
pub struct OutputZone {
    x: f32,
    y: f32,
    radius: f32,
    energy: f32,
}

impl OutputZone {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        OutputZone {
            x,
            y,
            radius,
            energy: 1000f32,
        }
    }
}
#[derive(Clone)]
pub struct Zone {
    x: f32,
    y: f32,
    radius: f32,
}
impl Zone {
    pub fn new(radius: f32) -> Self {
        Zone {
            x: rand::random::<f32>() * radius,
            y: rand::random::<f32>() * radius,
            radius: rand::random::<f32>() * radius * 0.1,
        }
    }

    fn get_x(self) -> f32 {
        self.x
    }
    fn set_x(mut self, x: f32) {
        self.x = x;
    }
    fn get_y(self) -> f32 {
        self.y
    }
    fn set_y(mut self, y: f32) {
        self.y = y
    }
    fn get_radius(self) -> f32 {
        self.radius
    }
    fn set_radius(mut self, radius: f32) {
        self.radius = radius;
    }
    pub fn nearby(&self, other: &Zone) -> bool {
        let dist = self.radius + other.radius;
        if (self.x - other.x).abs() < dist && (self.y - other.y).abs() < dist {
            return true;
        }
        false
    }
    // TODO Zone特型
    pub fn nearby_output(&self, other: &OutputZone) -> bool {
        let dist = self.radius + other.radius;
        if (self.x - other.x).abs() < dist && (self.y - other.y).abs() < dist {
            return true;
        }
        false
    }
    pub fn round_x(&self) -> f32 {
        self.x.round()
    }

    pub fn round_y(&self) -> f32 {
        self.y.round()
    }

    pub fn from(mut self, other: Zone) {
        self.x = other.x;
        self.y - other.y;
    }
}
pub const fn new_zone(x: f32, y: f32, radius: f32) -> Zone {
    Zone { x, y, radius }
}

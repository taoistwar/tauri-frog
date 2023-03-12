use rand::Rng;

use crate::{
    config::{ENV_FOOD_QTY, ENV_X_SIZE, ENV_Y_SIZE},
    egg::Egg,
    frog::Frog,
};

pub struct Env {
    eggs: Vec<Egg>,
    frogs: Vec<Frog>,
    foods: [[u8; ENV_X_SIZE]; ENV_Y_SIZE],
}

impl Env {
    pub fn new(eggs: Vec<Egg>) -> Self {
        let frogs: Vec<Frog> = Vec::new();
        let foods = [[0; ENV_X_SIZE]; ENV_Y_SIZE];
        let mut env = Env { eggs, frogs, foods };
        env.rebuild_frog_and_food();
        env
    }
    pub fn rebuild_frog_and_food(&mut self) {
        let mut rng = rand::thread_rng();
        // 先清空，再重新生成青蛙
        self.frogs.clear();
        for egg in &self.eggs {
            let x: usize = (ENV_X_SIZE) / 2 + &rng.gen_range(1..90);
            let y: usize = (ENV_Y_SIZE) / 2 + &rng.gen_range(1..90);
            self.frogs.push(Frog::new(x, y, egg.clone()));
        }
        for i in 0..ENV_X_SIZE {
            for j in 0..ENV_Y_SIZE {
                self.foods[i][j] = 0;
            }
        }
        // 先清空，再重新生成食物
        let mut foods = [[0; ENV_X_SIZE]; ENV_Y_SIZE];
        for _ in 1..ENV_FOOD_QTY {
            let x = rng.gen_range(1..ENV_X_SIZE);
            let y = rng.gen_range(1..ENV_Y_SIZE);
            foods[x][y] = 1;
        }
        self.foods = foods;
    }

    pub fn run(&mut self) {
        self.rebuild_frog_and_food();
    }
}

#[cfg(test)]
mod tests {
    use crate::egg::load_eggs;

    use super::Env;

    #[test]
    fn test_env() {
        let eggs = load_eggs();
        let mut env = Env::new(eggs);
        env.rebuild_frog_and_food();
    }
}

use crate::{
    config::{EGG_CELL_GROUP_QTY, ENV_EGG_QTY, FROG_BRAIN_RADIUS},
    frog::CellGroup,
};

#[derive(Clone)]
pub struct Egg {
    brain_radius: f32,
    cell_groups: Vec<CellGroup>,
}

impl Egg {
    pub fn new() -> Egg {
        let mut cell_groups = Vec::new();
        for _ in 0..EGG_CELL_GROUP_QTY {
            cell_groups.push(CellGroup::new());
        }
        let egg = Egg {
            brain_radius: FROG_BRAIN_RADIUS,
            cell_groups,
        };
        egg
    }
    pub fn get_brain_radius(&self) -> f32 {
        self.brain_radius
    }
    pub fn get_cell_groups(&self) -> &Vec<CellGroup> {
        &self.cell_groups
    }
}

pub fn load_eggs() -> Vec<Egg> {
    let mut eggs = Vec::new();
    for _ in 0..ENV_EGG_QTY {
        eggs.push(Egg::new())
    }
    eggs
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_range() {
        assert_eq!(10, (0..10).len());
    }
    #[test]
    fn test_rand() {
        let x: i32 = rand::random();
        println!("{}", x);
    }
    #[test]
    fn test_rng() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen_range(1..7);
    }
}

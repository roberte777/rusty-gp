use crate::Genotype;

pub enum GenerationMethod {
    RampedHalfAndHalf,
    Full,
}

pub struct TreeGenotype {
    pub fitness: f64,
    pub tree: Tree,
}
impl TreeGenotype {
    pub fn new(method: GenerationMethod, max_depth: u32) -> Self {
        Self {
            fitness: 0.0,
            tree: Tree::new(),
        }
    }
}
impl Genotype for TreeGenotype {
    fn fitness(&self) -> f64 {
        self.fitness
    }
    fn mutate(&mut self) {
        todo!()
    }

    fn crossover(&self, other: &Self) -> Self {
        todo!()
    }
}
pub struct Tree {}
impl Tree {
    fn new() -> Self {
        Tree {}
    }
}
impl Tree {}
pub trait Node {
    fn evaluate(&self) -> f64;
}

pub struct InternalNode {
    pub op: fn(f64, f64) -> f64,
    pub left: Option<Box<dyn Node>>,
    pub right: Option<Box<dyn Node>>,
}
impl InternalNode {
    fn new(op: fn(f64, f64) -> f64) -> Self {
        InternalNode {
            op,
            left: None,
            right: None,
        }
    }
}

impl Node for InternalNode {
    fn evaluate(&self) -> f64 {
        (self.op)(
            self.left.as_ref().unwrap().evaluate(),
            self.right.as_ref().unwrap().evaluate(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tree() {
        let genotype = TreeGenotype::new(GenerationMethod::Full, 5);
        // assert_eq!(tree.evaluate(), 0.0);
    }
}

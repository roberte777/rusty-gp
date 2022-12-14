use crate::Genotype;

pub struct TreeGenotype {
    pub fitness: f64,
    pub tree: Tree,
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
impl Tree {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tree() {}
}

pub mod tree;
pub trait Genotype {
    fn fitness(&self) -> f64;
    fn mutate(&mut self);
    fn crossover(&self, other: &Self) -> Self;
}
pub struct Population<T>
where
    T: Genotype,
{
    pub individuals: Vec<T>,
    pub init: fn(),
    pub parent_selection: fn(&Vec<T>) -> Vec<T>,
    pub survival_selection: fn(&Vec<T>) -> Vec<T>,
}
impl<T> Population<T>
where
    T: Genotype,
{
    pub fn new() -> PopulationBuilder<T> {
        PopulationBuilder {
            individuals: Vec::new(),
            init: None,
            parent_selection: None,
            survival_selection: None,
        }
    }
    pub fn generate_children(&mut self) {
        let parents = (self.parent_selection)(&self.individuals);
        let mut children = Vec::new();
        for i in 0..parents.len() {
            for j in 0..parents.len() {
                if i != j {
                    children.push(parents[i].crossover(&parents[j]));
                }
            }
        }
        for child in children {
            self.individuals.push(child);
        }
    }
    pub fn popualation_evalutation(&mut self) {
        self.individuals
            .sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap());
    }
    pub fn survival(&mut self) {
        (self.survival_selection)(&mut self.individuals);
    }
}
pub struct PopulationBuilder<T>
where
    T: Genotype,
{
    pub individuals: Vec<T>,
    pub init: Option<fn()>,
    pub parent_selection: Option<fn(&Vec<T>) -> Vec<T>>,
    pub survival_selection: Option<fn(&Vec<T>) -> Vec<T>>,
}
impl<T> PopulationBuilder<T>
where
    T: Genotype,
{
    pub fn new() -> PopulationBuilder<T> {
        todo!()
    }
    pub fn init(mut self, init: fn()) -> PopulationBuilder<T> {
        self.init = Some(init);
        self
    }
    pub fn parent_selection(mut self, parent_selection: fn(&Vec<T>) -> Vec<T>) -> Self {
        self.parent_selection = Some(parent_selection);
        self
    }
    pub fn survival_selection(mut self, survival_selection: fn(&Vec<T>) -> Vec<T>) -> Self {
        self.survival_selection = Some(survival_selection);
        self
    }
    pub fn build(self) -> Population<T> {
        Population {
            individuals: self.individuals,
            init: self.init.unwrap(),
            parent_selection: self.parent_selection.unwrap(),
            survival_selection: self.survival_selection.unwrap(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pop = Population::new()
            .init(|| {})
            .parent_selection(|_| vec![])
            .survival_selection(|_| vec![])
            .build();
    }
}

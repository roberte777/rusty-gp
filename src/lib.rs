pub mod tree;
pub trait Genotype<C> {
    fn fitness(&self) -> f64;
    fn evaluate(&self, context: Option<&C>) -> f64;
    fn mutate(&mut self);
    fn crossover(&self, other: &Self) -> Self;
}
pub struct Population<'a, T, C>
where
    T: Genotype<C>,
{
    pub individuals: Vec<T>,
    pub init: fn(),
    pub parent_selection: fn(&Vec<T>) -> Vec<T>,
    pub survival_selection: fn(&Vec<T>) -> Vec<T>,
    pub context: Option<&'a C>,
}
impl<'a, T, C> Population<'a, T, C>
where
    T: Genotype<C>,
{
    pub fn new() -> PopulationBuilder<'a, T, C> {
        PopulationBuilder {
            individuals: Vec::new(),
            init: None,
            parent_selection: None,
            survival_selection: None,
            context: None,
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
        self.individuals.iter().for_each(|individual| {
            individual.evaluate(self.context);
        });
    }
    pub fn survival(&mut self) {
        (self.survival_selection)(&mut self.individuals);
    }
}
pub struct PopulationBuilder<'a, T, C>
where
    T: Genotype<C>,
{
    pub individuals: Vec<T>,
    pub init: Option<fn()>,
    pub parent_selection: Option<fn(&Vec<T>) -> Vec<T>>,
    pub survival_selection: Option<fn(&Vec<T>) -> Vec<T>>,
    pub context: Option<&'a C>,
}
impl<'a, T, C> PopulationBuilder<'a, T, C>
where
    T: Genotype<C>,
{
    pub fn init(mut self, init: fn()) -> PopulationBuilder<'a, T, C> {
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
    pub fn context(mut self, context: &'a C) -> Self {
        self.context = Some(context);
        self
    }
    pub fn build(self) -> Population<'a, T, C> {
        Population {
            individuals: self.individuals,
            init: self.init.unwrap(),
            parent_selection: self.parent_selection.unwrap(),
            survival_selection: self.survival_selection.unwrap(),
            context: self.context,
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

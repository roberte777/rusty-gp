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
impl<C> Genotype<C> for TreeGenotype {
    fn fitness(&self) -> f64 {
        self.fitness
    }
    fn mutate(&mut self) {
        todo!()
    }

    fn crossover(&self, other: &Self) -> Self {
        todo!()
    }
    fn evaluate(&self, context: Option<&C>) -> f64 {
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
pub trait Node<C> {
    fn evaluate(&self, context: Option<&C>) -> f64;
}

pub struct InternalNode<C> {
    pub op: fn(context: Option<&C>, f64, f64) -> f64,
    pub left: Option<Box<dyn Node<C>>>,
    pub right: Option<Box<dyn Node<C>>>,
}
impl<C> InternalNode<C> {
    fn new(op: fn(context: Option<&C>, f64, f64) -> f64) -> Self {
        InternalNode {
            op,
            left: None,
            right: None,
        }
    }
}
impl<C> Node<C> for InternalNode<C> {
    fn evaluate(&self, context: Option<&C>) -> f64 {
        (self.op)(
            context,
            self.left.as_ref().unwrap().evaluate(context),
            self.right.as_ref().unwrap().evaluate(context),
        )
    }
}
pub struct UnaryNode<C> {
    pub value: fn(context: Option<&C>) -> f64,
}
impl<C> UnaryNode<C> {
    fn new(value: fn(context: Option<&C>) -> f64) -> Self {
        UnaryNode { value }
    }
}
impl<C> Node<C> for UnaryNode<C> {
    fn evaluate(&self, context: Option<&C>) -> f64 {
        (self.value)(context)
    }
}
impl ValueNode {
    fn new(value: f64) -> Self {
        ValueNode { value }
    }
}
pub struct ValueNode {
    pub value: f64,
}
impl<C> Node<C> for ValueNode {
    fn evaluate(&self, _context: Option<&C>) -> f64 {
        self.value
    }
}
pub struct RandomNode {
    pub value: f64,
}
impl RandomNode {
    fn new() -> Self {
        RandomNode {
            value: rand::random(),
        }
    }
}
impl<C> Node<C> for RandomNode {
    fn evaluate(&self, _context: Option<&C>) -> f64 {
        self.value
    }
}
pub enum LeafNodes<C> {
    Unary(UnaryNode<C>),
    Value(ValueNode),
    Random(RandomNode),
}

// TODO: How can I modify this code to make random nodes. These are nodes
// that create a random value on instantiation that is used for every evaluation

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tree() {
        let genotype = TreeGenotype::new(GenerationMethod::Full, 5);
        // assert_eq!(tree.evaluate(), 0.0);
    }
}

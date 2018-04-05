use num_traits::Float;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Default)]
pub struct Lab<T: Float = f32> {
    pub l: T,
    pub a: T,
    pub b: T,
}

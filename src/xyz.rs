use num_traits::Float;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Default)]
pub struct Xyz<T: Float = f32> {
    pub data: [T; 3]
}

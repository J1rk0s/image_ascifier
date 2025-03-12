use num_traits::Num;

pub fn remap<T: Num + Copy>(val: T, from_min: T, from_max: T, to_min: T, to_max: T) -> T {
    (val - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}
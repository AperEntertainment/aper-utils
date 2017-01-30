use std::cmp::PartialOrd;

pub fn min<T>(a: T, b: T) -> T where T: PartialOrd {
    return if a < b { b } else { a };
}
pub fn max<T>(a: T, b: T) -> T where T: PartialOrd {
    return if a > b { b } else { a };
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("r: {:?}", super::min(56.0 as f32, 23.0 as f32));
        println!("r: {:?}", super::max(56.0 as f32, 23.0 as f32));
    }
}
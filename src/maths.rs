use std::cmp::PartialOrd;


pub fn min<T>(a: T, b: T) -> T where T: PartialOrd
{
    return if a > b { b } else { a };
}
pub fn max<T>(a: T, b: T) -> T where T: PartialOrd
{
    return if a < b { b } else { a };
}

pub fn map(v: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32
{
    let a1_ = min(a1, a2);
    let a2_ = max(a1, a2);
    let b1_ = min(b1, b2);
    let b2_ = max(b1, b2);
    return b1_ + (v - a1_) * ((b2_ - b1_) / (a2_ - a1_));
}

pub fn normalize(a: f32, b: f32, c: f32) -> (f32, f32, f32)
{
    let t = (a * a + b * b + c * c).sqrt();
    return (a / t, b / t, c / t);
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("");
        println!("");
        println!("");
        println!("<=== BEGIN TESTS ===>");
        println!("");

        println!(" min(56.0, 23.0): {:?}", super::min(56.0 as f32, 23.0 as f32));
        println!(" max(56.0, 23.0): {:?}", super::max(56.0 as f32, 23.0 as f32));
        println!(" max(1280.5, 1253.0): {:?}", super::max(1280.5, 1253.0));
        println!(" map(2, 0, 10, 1, 2): (1.2) {:?}", super::map(2.0, 0.0, 10.0, 1.0, 2.0));
        println!(" normalize(1.5, 1.0, 0.0): (0.83, 0.55, 0.0) {:?}", super::normalize(1.5, 1.0, 0.0));

        println!("");
        println!("<=== END TESTS ===>");
        println!("");
        println!("");
        println!("");
    }
}
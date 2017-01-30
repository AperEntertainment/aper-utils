mod maths {
    pub fn min(a: f32, b: f32) -> f32 {
        return if a > b { b } else { a };
    }
    pub fn min(a: f32, b: f32) -> f32 {
        return if a < b { b } else { a };
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{}", super::min(56.0 as f32, 23.0 as f32));
    }
}
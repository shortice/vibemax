mod protocol;

pub fn vibemax<T: ToString>(x: Vec<T>) -> i64 {
    protocol::generate_answer(&x)
}

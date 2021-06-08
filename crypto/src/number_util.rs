use std::vec;

#[cfg(target_arch = "wasm32")]
pub fn random_iv(len: usize) -> Vec<u8> {
    use getrandom::getrandom;
    let mut v = vec![0u8; len];
    getrandom(&mut v);
    v
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_iv(len: usize) -> Vec<u8> {
    use rand::{thread_rng, RngCore};
    let mut v = vec![0u8; len];
    thread_rng().fill_bytes(&mut v);
    v
}

#[cfg(test)]
mod tests {
    use crate::number_util::random_iv;

    #[test]
    fn test_random() {
        let ret = random_iv(32);
        assert_eq!(32, ret.len());

        let ret = random_iv(64);
        assert_eq!(64, ret.len());
    }
}

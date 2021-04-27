mod generated;
pub use generated::api as api;
pub use generated::param as param;
pub use generated::ethereum as ethereum;

pub mod entry;
pub mod private_key;
pub mod public_key;
pub mod coin;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

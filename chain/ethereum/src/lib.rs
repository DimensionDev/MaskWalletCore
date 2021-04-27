pub mod address;
mod address_checksum;
pub mod entry;
mod signer;
mod transaction;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

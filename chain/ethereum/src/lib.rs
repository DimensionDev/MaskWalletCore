pub mod address;
pub mod entry;
mod address_checksum;
mod signer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

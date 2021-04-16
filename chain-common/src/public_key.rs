use crypto::public_key::PublicKeyType;

pub struct PublicKey {
    pub r#type: PublicKeyType,
    data: Vec<u8>,
}

impl PublicKey {
    pub fn new(r#type: PublicKeyType, data: &[u8]) -> Self {
        PublicKey {
            r#type: r#type,
            data: data.to_vec()
        }
    }
}
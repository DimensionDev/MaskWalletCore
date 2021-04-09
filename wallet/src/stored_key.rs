use uuid::Uuid;

pub enum StoredKeyType {
    PrivateKey,
    Mnemonic,
}

pub struct StoredKey {
    r#type: StoredKeyType,

    id: String,

    name: String,
}

impl StoredKey {
    pub fn create_with_private_key(name: &str, password: &str, private_key: &str) -> Self {
        let uuid = Uuid::new_v4();
        StoredKey {
            r#type: StoredKeyType::PrivateKey,
            name: String::from(name),
            id: uuid.to_string(),
        }
    }
}
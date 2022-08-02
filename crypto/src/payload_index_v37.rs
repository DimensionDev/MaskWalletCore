use std::convert::TryFrom;

#[allow(dead_code)]
#[repr(i64)]
#[derive(Debug, PartialEq, Eq)]
pub enum Index {
    Version = 0,
    AuthorNetwork = 1,
    AuthorID = 2,
    AuthorPublicKeyAlgorithm = 3,
    AuthorPublicKey = 4,
    Encryption = 5,
    Data = 6,
}

impl TryFrom<i64> for Index {
    type Error = ();
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            x if x == Index::Version as i64 => Ok(Index::Version),
            x if x == Index::AuthorNetwork as i64 => Ok(Index::AuthorNetwork),
            x if x == Index::AuthorID as i64 => Ok(Index::AuthorID),
            x if x == Index::AuthorPublicKeyAlgorithm as i64 => Ok(Index::AuthorPublicKeyAlgorithm),
            x if x == Index::AuthorPublicKey as i64 => Ok(Index::AuthorPublicKey),
            x if x == Index::Encryption as i64 => Ok(Index::Encryption),
            x if x == Index::Data as i64 => Ok(Index::Data),
            _ => Err(()),
        }
    }
}

use std::string::ToString;
use std::fmt::Debug;
use serde::{ Serialize, Deserialize };

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct DerivationPath {
    indices: Vec<DerivationPathIndex>,
}

impl DerivationPath {
    pub fn new(path: &str) -> Result<DerivationPath, Error> {
        let children: Vec<&str> = path.split('/').into_iter().collect();
        let mut indices: Vec<DerivationPathIndex> = vec![];
        for child in children {
            if child == "m" {
                continue;
            }
            let child_str: &str;
            let hardened = child.ends_with('\'');
            if hardened {
                child_str = child.strip_suffix("'").unwrap();
            } else {
                child_str = child;
            }
            let child_index = match child_str.parse::<u32>() {
                Ok(child_index) => child_index,
                Err(_) => return Err(Error::InvalidDerivationpath)
            };
            indices.push(DerivationPathIndex{
                value: child_index,
                hardened
            });
        }
        Ok(DerivationPath{
            indices
        })
    }
}

impl ToString for DerivationPath {
    fn to_string(&self) -> String {
        let mut path = String::from("m/");
        self.indices.iter().for_each(|index| {
            path.push_str(&index.value.to_string());
            if index.hardened {
                path.push('\'');
            };
            path.push('/');
        });
        if path.ends_with('/') {
            path.strip_suffix("/").unwrap().to_owned()
        } else {
            path
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DerivationPathIndex {
    value: u32,
    hardened: bool,
}

impl ToString for DerivationPathIndex {
    fn to_string(&self) -> String {
        if self.hardened {
            format!("{}'", self.value)
        } else {
            format!("{}", self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::derivation_path::*;
    use crate::Error;
    #[test]
    fn test_derivation_path_parse() {
        let invalid_test_path_str = "m/m44'/60'/0'/0/0";
        assert_eq!(DerivationPath::new(&invalid_test_path_str).unwrap_err(), Error::InvalidDerivationpath);

        let test_path_str = "m/44'/60'/0'/0/0";
        let derivation_path = DerivationPath::new(&test_path_str).expect("fail to parse test derivation path str");
        assert_eq!(derivation_path.to_string(), test_path_str);
    }
}

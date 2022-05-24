use bincode;
use crypto::{digest::Digest, sha3::Sha3};
use serde::{Deserialize, Serialize};

pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize
{
    bincode::serialize(value).unwrap()
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

// 通过 crypt:sha3 将 byte 转成 hash string
pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();

    hasher.input(value);

    hasher.result_str()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serialize() {
        let value = Point { x: 0, y: 0 };

        let se = my_serialize(&value);

        let de: Point = my_deserialize(&se);

        assert_eq!(value, de);
    }
}

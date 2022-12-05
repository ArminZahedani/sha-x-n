use sha2::{Digest, Sha512};
use std::collections::HashMap;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn sha_512_n(data: &str, n: usize) -> Vec<u8> {
    let mut hasher = Sha512::new();

    hasher.update(data);

    let result = hasher.finalize();

    result.get(..n / 8).unwrap().to_owned()
}

pub fn collider(n: usize) {
    let mut seen: HashMap<Vec<u8>, String> = HashMap::new();

    loop {
        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let result = sha_512_n(&rand_string, n);

        if seen.contains_key(&result) {
            //println!("FOUND COLLISION between {:?} and {:?}", rand_string, seen.get(&result).unwrap());
            break;
        }
        seen.insert(result, rand_string);
    }
}

pub fn pre_image(image: Vec<u8>) {
    let mut counter = 0;
    loop {
        counter += 1;
        // https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let result = sha_512_n(&rand_string, image.len() * 8);
        if result == image {
            println!("string {:?} hashes to {:?}. Found in {counter} tries", rand_string, hex::encode(result));
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn sanity_check8() {
        let hash = sha_512_n("a", 8);
        let real = hex::decode("1f").unwrap();
        assert_eq!(hash, real);
    }

    #[test]
    fn sanity_check16() {
        let hash = sha_512_n("a", 16);
        let real = hex::decode("1f40").unwrap();
        assert_eq!(hash, real);
    }

    #[test]
    fn sanity_check24() {
        let hash = sha_512_n("a", 24);
        let real = hex::decode("1f40fc").unwrap();
        assert_eq!(hash, real);
    }

    #[test]
    fn sanity_check32() {
        let hash = sha_512_n("a", 32);
        let real = hex::decode("1f40fc92").unwrap();
        assert_eq!(hash, real);
    }

    #[test]
    fn sanity_check40() {
        let hash = sha_512_n("a", 40);
        let real = hex::decode("1f40fc92da").unwrap();
        assert_eq!(hash, real);
    }

    #[test]
    fn sanity_check48() {
        let hash = sha_512_n("a", 48);
        let real = hex::decode("1f40fc92da24").unwrap();
        assert_eq!(hash, real);
    }
    #[test]
    fn collider8() {
        collider(8);
    }

    #[test]
    fn pre_image16() {
        pre_image(hex::decode("3D4B").unwrap());
    }
}

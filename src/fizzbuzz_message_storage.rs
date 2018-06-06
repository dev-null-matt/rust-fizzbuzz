use std::collections::HashMap;

///
/// A storage for messages used to format fizzbuzz numbers.
///
pub struct FizzbuzzMessageStorage {
    map : HashMap<i64, String>,
}

impl FizzbuzzMessageStorage {

    ///
    /// Constructs a default fizzbuzz message store consisting of:
    ///
    /// * fizz
    /// * buzz
    /// * fizzbuzz
    ///
    pub fn default() -> FizzbuzzMessageStorage {

        let mut map = HashMap::new();
        map.insert(3, "fizz".to_string());
        map.insert(5, "buzz".to_string());
        map.insert(15, "fizzbuzz".to_string());

        FizzbuzzMessageStorage {
            map: map,
        }
    }

    ///
    /// Constructs a fizzbuzz message store with the supplied messages.
    ///
    pub fn new(fizz : &str, buzz : &str, fizzbuzz : &str) -> FizzbuzzMessageStorage {

        let mut map = HashMap::new();
        map.insert(3, fizz.to_owned());
        map.insert(5, buzz.to_owned());
        map.insert(15, fizzbuzz.to_owned());

        FizzbuzzMessageStorage {
            map: map,
        }
    }

    ///
    /// Returns the fizz message for this message storage.
    ///
    pub fn fizz(&self) -> String {
        self.map.get(&3).unwrap_or(&"fizz".to_string()).to_owned()
    }

    ///
    /// Returns the buzz message for this message storage.
    ///
    pub fn buzz(&self) -> String {
        self.map.get(&5).unwrap_or(&"buzz".to_string()).to_owned()
    }

    ///
    /// Returns the fizzbuzz message for this message storage.
    ///
    pub fn fizzbuzz(&self) -> String {
        self.map.get(&15).unwrap_or(&"fizzbuzz".to_string()).to_owned()
    }
}

#[test]
fn test_default_constructor() {

    let messages : FizzbuzzMessageStorage = FizzbuzzMessageStorage::default();

    assert_eq!("fizz".to_string(), messages.fizz());
    assert_eq!("buzz".to_string(), messages.buzz());
    assert_eq!("fizzbuzz".to_string(), messages.fizzbuzz());
}

#[test]
fn test_explicit_constructor() {

    let messages : FizzbuzzMessageStorage = FizzbuzzMessageStorage::new("fizzle", "buzzle", "fizzbuzzle");

    assert_eq!("fizzle".to_string(), messages.fizz());
    assert_eq!("buzzle".to_string(), messages.buzz());
    assert_eq!("fizzbuzzle".to_string(), messages.fizzbuzz());
}

///
/// A storage for messages used to format fizzbuzz numbers.
///
pub struct FizzbuzzMessageStorage {
    fizz        : String,
    buzz        : String,
    fizzbuzz    : String,
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
        FizzbuzzMessageStorage {
            fizz : "fizz".to_string(),
            buzz : "buzz".to_string(),
            fizzbuzz : "fizzbuzz".to_string(),
        }
    }

    ///
    /// Constructs a fizzbuzz message store with the supplied messages.
    ///
    pub fn new(fizz : &str, buzz : &str, fizzbuzz : &str) -> FizzbuzzMessageStorage {
        FizzbuzzMessageStorage {
            fizz : fizz.to_string(),
            buzz : buzz.to_string(),
            fizzbuzz : fizzbuzz.to_string(),
        }
    }

    ///
    /// Returns the fizz message for this message storage.
    ///
    pub fn fizz(&self) -> String {
        self.fizz.to_string()
    }

    ///
    /// Returns the buzz message for this message storage.
    ///
    pub fn buzz(&self) -> String {
        self.buzz.to_string()
    }

    ///
    /// Returns the fizzbuzz message for this message storage.
    ///
    pub fn fizzbuzz(&self) -> String {
        self.fizzbuzz.to_string()
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

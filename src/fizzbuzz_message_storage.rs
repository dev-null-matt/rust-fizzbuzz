pub struct FizzbuzzMessageStorage {
    fizz        : String,
    buzz        : String,
    fizzbuzz    : String,
}

impl FizzbuzzMessageStorage {

    pub fn default() -> FizzbuzzMessageStorage {
        FizzbuzzMessageStorage {
            fizz : "fizz".to_string(),
            buzz : "buzz".to_string(),
            fizzbuzz : "fizzbuzz".to_string(),
        }
    }

    pub fn new(fizz : &str, buzz : &str, fizzbuzz : &str) -> FizzbuzzMessageStorage {
        FizzbuzzMessageStorage {
            fizz : fizz.to_string(),
            buzz : buzz.to_string(),
            fizzbuzz : fizzbuzz.to_string(),
        }
    }

    pub fn fizz(&self) -> String {
        self.fizz.to_string()
    }

    pub fn buzz(&self) -> String {
        self.buzz.to_string()
    }

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

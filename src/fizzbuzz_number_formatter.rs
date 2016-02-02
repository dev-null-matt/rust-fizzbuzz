use fizzbuzz_message_storage::FizzbuzzMessageStorage;

///
/// Formatter for fizzbuzz numbers.  Takes a number and uses
/// the supplied message storage to format the supplied number.
///
pub struct FizzbuzzMessageFormatter {
    message_store : FizzbuzzMessageStorage,
}

impl FizzbuzzMessageFormatter {

    ///
    /// Constructs a `FizzbuzzMessageFormatter` with a default `FizzbuzzMessageStorage`.
    ///
    pub fn default() -> FizzbuzzMessageFormatter {
        FizzbuzzMessageFormatter {
            message_store : FizzbuzzMessageStorage::default(),
        }
    }

    ///
    /// Formats a fizzbuzz number using this formatter's underlying message storage.
    ///
    pub fn format_number(&self, count : i64) -> String {
        let message =
            if count % 15 == 0 {
                self.message_store.fizzbuzz()
            } else if count % 5 == 0 {
                self.message_store.buzz()
            } else if count % 3 == 0 {
                self.message_store.fizz()
            } else {
                count.to_string()
            };
        message
    }
}

#[test]
fn test_format_fizzbuzz() {
    let formatter = FizzbuzzMessageFormatter::default();
    assert_eq!("fizzbuzz".to_string(), formatter.format_number(45));
}

#[test]
fn test_format_fizz() {
    let formatter = FizzbuzzMessageFormatter::default();
    assert_eq!("fizz".to_string(), formatter.format_number(9));
}

#[test]
fn test_format_buzz() {
    let formatter = FizzbuzzMessageFormatter::default();
    assert_eq!("buzz".to_string(), formatter.format_number(25));
}

#[test]
fn test_format_nonfizz_nonbuzz() {
    let formatter = FizzbuzzMessageFormatter::default();
    assert_eq!("14".to_string(), formatter.format_number(14));
}

// The idea is that warnings are a very user focussed string, rather than complex nested validation.

// Where validation tells the user there is fundamentally a problem that means the RD300NX object is invalid,
// a warning only indicates that although valid, the configuration provided may not operate as expected
// in some scenarios.

pub trait Warnings {
    fn warnings(&self) -> Vec<String>;
}
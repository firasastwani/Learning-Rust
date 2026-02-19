// When would you take `&String` instead of `&str`?
//
// Usually you want `&str` (more flexible). These are the main situations
// where `&String` is used on purpose.

// -----------------------------------------------------------------------------
// 1. You're wrapping a legacy or external API that only accepts &String
// -----------------------------------------------------------------------------

fn legacy_api_that_takes_ref_string(_s: &String) {
    // e.g. old crate or FFI that was written to take &String
}

fn your_function(s: &String) {
    legacy_api_that_takes_ref_string(s);
}

// -----------------------------------------------------------------------------
// 2. You want to enforce "caller must pass an owned String" (no literals)
// -----------------------------------------------------------------------------
// Example: a cache that only accepts heap-allocated strings (e.g. so you can
// clear or shrink them later). Taking &String rejects string literals.

use std::collections::HashSet;

struct StringCache {
    entries: HashSet<String>,
}

impl StringCache {
    fn add(&mut self, s: &String) {
        // Caller must pass &String — so we know we're not storing a &'static str.
        // We clone to own it; the point is the *API* only accepts owned Strings.
        self.entries.insert(s.clone());
    }
}

// -----------------------------------------------------------------------------
// 3. You need to pass the same reference to something that takes &String
//    and you want to avoid the caller having to do the conversion
// -----------------------------------------------------------------------------

fn process_and_log(msg: &String) {
    // Imagine we need to pass msg to several places that take &String.
    legacy_api_that_takes_ref_string(msg);
    println!("Logged: {}", msg);
}

fn main() {
    // With &String, only this works:
    let owned = String::from("hello");
    your_function(&owned);
    process_and_log(&owned);

    // This would NOT compile if we only had &String APIs:
    // process_and_log("literal");  // type mismatch: &str vs &String

    // With &str, both would work:
    // process_and_log("literal");
    // process_and_log(&owned);

    let mut cache = StringCache {
        entries: HashSet::new(),
    };
    let key = String::from("user:1001");
    cache.add(&key);
    println!("Cache has {} entries", cache.entries.len());
}
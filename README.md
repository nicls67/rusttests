# rusttests

`rusttests` is a Rust crate designed to facilitate the testing process by providing helper functions to check variable values and method return statuses (like `Some`, `None`, `Ok` or `Err`).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rusttests = "1.1.0"
```

## Usage

Import the necessary functions and types from the crate:

```rust
use rusttests::{check_result, check_option, check_value, check_struct, CheckType};
```

Each check must be identified by an ID represented as a tuple `(u8, u8)`. This ID aids in identifying the specific check that failed.

### Checking Results (`Result<T, E>`)

`check_result` verifies if a `Result` is `Ok` or `Err`. If `Ok` is expected and obtained, it returns `Ok(Some(value))`, allowing further checks on the wrapped value.

```rust
# use rusttests::{check_result};
// Check if result is Ok
let obtained: Result<i32, String> = Ok(2);
let value = check_result((1, 1), obtained.clone(), true).unwrap();
// Ok is expected, test returns Ok. `value` is Some(2)
assert_eq!(value, Some(2));

// Check if result is Err
check_result((1, 2), obtained.clone(), false);
// Err is expected, checking against Ok(2) would result in an error
```

### Checking Options (`Option<T>`)

`check_option` verifies if an `Option` is `Some` or `None`.

```rust
# use rusttests::{check_option};
// Check if option is Some
let obtained = Some(5);
let value = check_option((2, 1), obtained, true).unwrap();
// Some is expected, test returns Ok. `value` is Some(5)
assert_eq!(value, Some(5));

// Check if option is None
check_option((2, 2), obtained, false);
// None is expected
```

### Checking Values

`check_value` compares two values using a specified `CheckType` (Equal, Different, Superior, etc.).

```rust
# use rusttests::{check_value, CheckType};
let obtained = 12;

// Check for equality
check_value((3, 1), &obtained, &12, CheckType::Equal);

// Check if obtained is inferior to 10
check_value((3, 2), &obtained, &10, CheckType::Inferior);
```

### Checking Structures

`check_struct` compares two structures for equality or difference.

```rust
# use rusttests::{check_struct, CheckType};
#[derive(Debug, PartialEq)]
struct MyStruct {
    a: i32,
}

let s1 = MyStruct { a: 1 };
let s2 = MyStruct { a: 1 };

check_struct((4, 1), &s1, &s2, CheckType::Equal);
```

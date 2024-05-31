# RustTests

RustTests is a crate designed to facilitate Rust code testing process.

It offers function to check variable value or method return status (like `Some`, `None`, `Ok` or `Err`)

## Usage

Each check must be identified by an ID represented as a tuple `(u8,u8)`. This ID will be shown in case of failed check in order to facilitate failed check identification.

`check_result` and `check_option` will unwrap the `Ok` and `Some` variants content and return it when the check is OK. This allows the tester to check the wrapped value in a second check.

`check_struct` has the same behaviour as `check_value`, except only equality check is available (`CheckType::Equal` and `CheckType::Different`)

```rust
use rusttests::{check_result, check_option, check_value, check_struct, CheckType};

// Check a return status (Ok or Err)
let obtained: Result<i32, String> = Ok(2);
let value = check_result((1,1), obtained.clone(), true).unwrap(); // Ok is expected, test will return Ok
assert_eq!(value, Some(2));

check_result((1,2), obtained.clone(), false); // Err is expected, test will return Err


// Check an Option (Some or None)
let obtained = Some(5);
let value = check_option((2,1), obtained, true).unwrap(); // Some is expected, test will return Ok
assert_eq!(value, Some(5));

check_option((2,2), obtained, false); // None is expected, test will return Err


// Check value
let obtained = 12;
check_value((3,1), &obtained, &12, CheckType::Equal); // Equality check, result is Ok
check_value((3,2), &obtained, &10, CheckType::Inferior); // Comparison check, result is Err
```

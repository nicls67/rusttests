//!
//! Integration tests for crate RustTests
//!

use rusttests::{check_result, check_option, check_struct, check_value, CheckType};

#[test]
fn result_1() -> Result<(), String> {
    let obtained: Result<i32, String> = Ok(3);
    match check_result((1,1), obtained, true) {
        Ok(res) => {
            if let Some(value) = res {
                if value == 3 {
                    Ok(())
                } else {
                    Err("Bad value".to_string())
                }
            } else { Err("Bad option".to_string())  }
        }
        Err(_) => Err("Bad result".to_string())
    }
}

#[test]
fn result_2() -> Result<(), String> {
    let obtained: Result<i32, String> = Ok(3);
    match check_result((2,2), obtained, false) {
        Ok(_) => Err("Bad result".to_string()),
        Err(e) => {
            if e == *"Step 2.2 : Obtained result should be Err, got Ok" {
                Ok(())
            } else { Err("Bad error message".to_string()) }
        }
    }
}

#[test]
fn option_1() -> Result<(), String> {
    let obtained: Option<i32> = Some(3);
    match check_option((5,1), obtained, true) {
        Ok(res) => {
            if let Some(value) = res {
                if value == 3 {
                    Ok(())
                } else {
                    Err("Bad value".to_string())
                }
            } else { Err("Bad option".to_string())  }
        }
        Err(_) => Err("Bad result".to_string())
    }
}

#[test]
fn option_2() -> Result<(), String> {
    let obtained: Option<i32> = None;
    match check_option((5,2), obtained, false) {
        Ok(res) => {
            if res.is_none() {
                Ok(())
            } else {
                Err("Bad option".to_string())
            }
        }
        Err(_) => Err("Bad result".to_string())
    }
}

#[test]
fn struct_1() -> Result<(), String> {
    #[derive(Debug, PartialEq)]
    struct TestStruct { a: i32 }
    let s1 = TestStruct { a: 1 };
    let s2 = TestStruct { a: 1 };

    check_struct((6,1), &s1, &s2, CheckType::Equal)
}

#[test]
fn struct_2() -> Result<(), String> {
    #[derive(Debug, PartialEq)]
    struct TestStruct { a: i32 }
    let s1 = TestStruct { a: 1 };
    let s2 = TestStruct { a: 2 };

    check_struct((6,2), &s1, &s2, CheckType::Different)
}

#[test]
fn value_1() -> Result<(), String> {
    let v1 = 10;
    let v2 = 10;
    check_value((7,1), &v1, &v2, CheckType::Equal)
}

#[test]
fn value_2() -> Result<(), String> {
    let v1 = 10;
    let v2 = 5;
    check_value((7,2), &v1, &v2, CheckType::Superior)
}

#[test]
fn result_3() -> Result<(), String> {
    let obtained: Result<i32, String> = Err("message".to_string());
    match check_result((2,3), obtained, false) {
        Ok(opt) => if opt.is_none() {Ok(())} else { Err("Bad option".to_string()) },
        Err(_) => Err("Bad result".to_string())
    }
}

#[test]
fn result_4() -> Result<(), String> {
    let obtained: Result<i32, String> = Err("message".to_string());
    match check_result((4,1), obtained, true) {
        Ok(_) => Err("Bad result".to_string()),
        Err(e) => {
            if e == *"Step 4.1 : Obtained result should be Ok, got Err. Error message is \"message\"" {
                Ok(())
            } else { Err("Bad error message".to_string()) }
        }
    }
}
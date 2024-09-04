//!
//! Integration tests for crate RustTests
//!

use rusttests::check_result;

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
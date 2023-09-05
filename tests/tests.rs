use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::ops;

use byteflags::*;

multiselect! {
    #[derive(Debug)]
    struct TestMultiSelect {
        TEST_A -> "Test A",
        TEST_B -> "Test B",
        TEST_C -> "Test C",
        TEST_D -> "Test D",
    }
}


// TODO!(Figure out how to make this compatible with smash feature branch)
// fn get_random_int(max: usize) -> usize {
//     rand::thread_rng().gen_range(0..max)
// }

#[test]
fn test_multiselect_serialize() -> Result<(), Box<dyn Error>> {
    let a = serde_json::to_string(&TestMultiSelect {
        TEST_A: 7,
        TEST_B: 7,
        TEST_C: 2,
        TEST_D: 3,
    })
    .unwrap();
    let b = "[7,7,2,3]".to_string();
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn test_multiselect_deserialize() -> Result<(), Box<dyn Error>> {
    let a = serde_json::from_str::<TestMultiSelect>("[5,3,3,1]").unwrap();
    let b = TestMultiSelect {
        TEST_A: 5,
        TEST_B: 3,
        TEST_C: 3,
        TEST_D: 1,
    };
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn test_multiselect_const() -> Result<(), Box<dyn Error>> {
    let a = TestMultiSelect::TEST_C;
    let b = TestMultiSelect {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 1,
        TEST_D: 0,
    };
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn test_multiselect_addition() -> Result<(), Box<dyn Error>> {
    let a = TestMultiSelect::TEST_A
        + TestMultiSelect::TEST_B
        + TestMultiSelect::TEST_B
        + TestMultiSelect::TEST_B
        + TestMultiSelect::TEST_B;
    let b = TestMultiSelect {
        TEST_A: 1,
        TEST_B: 4,
        TEST_C: 0,
        TEST_D: 0,
    };
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn test_multiselect_multiplication() -> Result<(), Box<dyn Error>> {
    let a = TestMultiSelect::TEST_C
        + TestMultiSelect::TEST_C
        + TestMultiSelect::TEST_D
        + TestMultiSelect::TEST_D
        + TestMultiSelect::TEST_D
        + TestMultiSelect::TEST_D;
    let b = TestMultiSelect {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 1,
        TEST_D: 2,
    } * 2;
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn test_multiselect_contains() -> Result<(), Box<dyn Error>> {
    let abcd = serde_json::from_str::<TestMultiSelect>("[1,2,0,0]").unwrap();
    let a = TestMultiSelect::TEST_A;
    let b = TestMultiSelect::TEST_A + TestMultiSelect::TEST_A;
    let c = TestMultiSelect::TEST_A * 5;
    let d = TestMultiSelect::TEST_B;
    let e = TestMultiSelect::TEST_C;
    assert!(abcd.contains(&a));
    assert!(abcd.contains(&b));
    assert!(abcd.contains(&c));
    assert!(abcd.contains(&d));
    assert!(!abcd.contains(&e));
    Ok(())
}

#[test]
fn test_multiselect_display() -> Result<(), Box<dyn Error>> {
    let a = format!("{}", TestMultiSelect::TEST_A);
    let b = "Test A";
    assert_eq!(a, b);
    let c = format!(
        "{}",
        serde_json::from_str::<TestMultiSelect>("[0,0,1,5]").unwrap()
    );
    let d = "Test C + Test D";
    assert_eq!(c, d);
    Ok(())
}

#[test]
fn test_multiselect_match() -> Result<(), Box<dyn Error>> {
    let a = TestMultiSelect::TEST_D;
    assert!(match a {
        TestMultiSelect::TEST_A => false,
        TestMultiSelect::TEST_B => false,
        TestMultiSelect::TEST_C => false,
        TestMultiSelect::TEST_D => true,
        _ => false,
    });
    Ok(())
}
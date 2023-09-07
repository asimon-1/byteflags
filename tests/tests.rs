#![allow(non_snake_case)]
use byteflags::*;

byteflags! {
    #[derive(Debug)]
    struct TestByteFlags {
        TEST_A -> "Test A",
        TEST_B -> "Test B",
        TEST_C -> "Test C",
        TEST_D -> "Test D",
    }
}

#[test]
fn test_byteflags_serialize() -> Result<(), String> {
    let a = serde_json::to_string(&TestByteFlags {
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
fn test_byteflags_deserialize() -> Result<(), String> {
    let a = serde_json::from_str::<TestByteFlags>("[5,3,3,1]").unwrap();
    let b = TestByteFlags {
        TEST_A: 5,
        TEST_B: 3,
        TEST_C: 3,
        TEST_D: 1,
    };
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn test_byteflags_const() -> Result<(), String> {
    let a = TestByteFlags::TEST_C;
    let b = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 1,
        TEST_D: 0,
    };
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn test_byteflags_addition() -> Result<(), String> {
    let a = TestByteFlags::TEST_A
        + TestByteFlags::TEST_B
        + TestByteFlags::TEST_B
        + TestByteFlags::TEST_B
        + TestByteFlags::TEST_B;
    let b = TestByteFlags {
        TEST_A: 1,
        TEST_B: 4,
        TEST_C: 0,
        TEST_D: 0,
    };
    assert_eq!(a, b);
    Ok(())
}

#[test]
fn test_byteflags_add_assign() -> Result<(), String> {
    let mut a = TestByteFlags::TEST_A;
    let b = TestByteFlags::TEST_B;
    let a_b = TestByteFlags {
        TEST_A: 1,
        TEST_B: 1,
        TEST_C: 0,
        TEST_D: 0,
    };
    let a_bb = TestByteFlags {
        TEST_A: 1,
        TEST_B: 2,
        TEST_C: 0,
        TEST_D: 0,
    };
    a += b;
    assert_eq!(a, a_b);
    a += b;
    assert_eq!(a, a_bb);

    let mut x = TestByteFlags {
        TEST_A: u8::MAX,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    let y = TestByteFlags::TEST_A;
    let x_y = TestByteFlags {
        TEST_A: u8::MAX,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    x += y;
    assert_eq!(x, x_y);
    Ok(())
}

#[test]
fn test_byteflags_subtraction() -> Result<(), String> {
    let a = TestByteFlags {
        TEST_A: 1,
        TEST_B: 4,
        TEST_C: 0,
        TEST_D: 0,
    };
    let b = TestByteFlags {
        TEST_A: 0,
        TEST_B: 1,
        TEST_C: 0,
        TEST_D: 0,
    };
    let a_b = TestByteFlags {
        TEST_A: 1,
        TEST_B: 3,
        TEST_C: 0,
        TEST_D: 0,
    };
    let a_bbbb = TestByteFlags {
        TEST_A: 1,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    let a_bbbbb = TestByteFlags {
        TEST_A: 1,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    let a_aabbbbb = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    assert_eq!(a - b, a_b);
    assert_eq!(a - (b + b + b + b), a_bbbb);
    assert_eq!(a - (b + b + b + b + b), a_bbbbb);
    assert_eq!(a - (a + a + b + b + b + b + b), a_aabbbbb);
    Ok(())
}

#[test]
fn test_byteflags_sub_assign() -> Result<(), String> {
    let mut a = TestByteFlags {
        TEST_A: 1,
        TEST_B: 2,
        TEST_C: 0,
        TEST_D: 0,
    };
    let b = TestByteFlags {
        TEST_A: 0,
        TEST_B: 1,
        TEST_C: 0,
        TEST_D: 0,
    };
    let a_b = TestByteFlags {
        TEST_A: 1,
        TEST_B: 1,
        TEST_C: 0,
        TEST_D: 0,
    };
    let a_bb = TestByteFlags {
        TEST_A: 1,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    a -= b;
    assert_eq!(a, a_b);
    a -= b;
    assert_eq!(a, a_bb);

    let mut x = TestByteFlags {
        TEST_A: 1,
        TEST_B: 1,
        TEST_C: 0,
        TEST_D: 0,
    };
    let y = TestByteFlags {
        TEST_A: 0,
        TEST_B: 1,
        TEST_C: 0,
        TEST_D: 0,
    };
    let z = TestByteFlags {
        TEST_A: 1,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    let x_y = TestByteFlags {
        TEST_A: 1,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    let x_y_z = TestByteFlags::new();
    x -= y;
    assert_eq!(x, x_y);
    x -= z;
    assert_eq!(x, x_y_z);
    Ok(())
}

#[test]
fn test_byteflags_mul_u8() -> Result<(), String> {
    let a = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 2,
        TEST_D: 4,
    };
    let b = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 1,
        TEST_D: 2,
    };
    assert_eq!(a, b * 2);
    Ok(())
}

#[test]
fn test_byteflags_mul_byteflags() -> Result<(), String> {
    let a = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 2,
        TEST_D: 3,
    };
    let b = TestByteFlags {
        TEST_A: 0,
        TEST_B: 10,
        TEST_C: 3,
        TEST_D: u8::MAX - 3,
    };
    let ab = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 6,
        TEST_D: u8::MAX,
    };
    assert_eq!(a * b, ab);
    Ok(())
}

#[test]
fn test_byteflags_mul_assign_byteflags() -> Result<(), String> {
    let mut a = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 2,
        TEST_D: 3,
    };
    let b = TestByteFlags {
        TEST_A: 0,
        TEST_B: 10,
        TEST_C: 3,
        TEST_D: u8::MAX - 3,
    };
    let ab = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 6,
        TEST_D: u8::MAX,
    };
    a *= b;
    assert_eq!(a, ab);
    Ok(())
}

#[test]
fn test_byteflags_mul_assign_u8() -> Result<(), String> {
    let mut a = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 1,
        TEST_D: 2,
    };
    let a2 = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 2,
        TEST_D: 4,
    };
    a *= 2;
    assert_eq!(a, a2);
    let mut b = TestByteFlags {
        TEST_A: u8::MAX,
        TEST_B: 1,
        TEST_C: 0,
        TEST_D: 0,
    };
    let b2 = TestByteFlags {
        TEST_A: u8::MAX,
        TEST_B: 2,
        TEST_C: 0,
        TEST_D: 0,
    };
    b *= 2;
    assert_eq!(b, b2);
    Ok(())
}

#[test]
fn test_byteflags_contains() -> Result<(), String> {
    let abcd = serde_json::from_str::<TestByteFlags>("[1,2,0,0]").unwrap();
    let a = TestByteFlags::TEST_A;
    let b = TestByteFlags::TEST_A * 2;
    let c = TestByteFlags::TEST_A * 5;
    let d = TestByteFlags::TEST_B;
    let e = TestByteFlags::TEST_C;
    assert!(abcd.contains(&a));
    assert!(abcd.contains(&b));
    assert!(abcd.contains(&c));
    assert!(abcd.contains(&d));
    assert!(!abcd.contains(&e));
    Ok(())
}

#[test]
fn test_byteflags_display() -> Result<(), String> {
    let a = format!("{}", TestByteFlags::TEST_A);
    let b = "Test A";
    assert_eq!(a, b);
    let c = format!(
        "{}",
        TestByteFlags {
            TEST_A: 0,
            TEST_B: 0,
            TEST_C: 1,
            TEST_D: 5,
        }
    );
    let d = "Test C + Test D";
    assert_eq!(c, d);
    Ok(())
}

#[test]
fn test_byteflags_match() -> Result<(), String> {
    let a = TestByteFlags::TEST_D;
    assert!(match a {
        TestByteFlags::TEST_A => false,
        TestByteFlags::TEST_B => false,
        TestByteFlags::TEST_C => false,
        TestByteFlags::TEST_D => true,
        _ => false,
    });
    Ok(())
}

#[test]
fn test_byteflags_new() -> Result<(), String> {
    let a = TestByteFlags::new();
    let b = TestByteFlags {
        TEST_A: 0,
        TEST_B: 0,
        TEST_C: 0,
        TEST_D: 0,
    };
    assert_eq!(a, b);
    Ok(())
}

#[test]
#[cfg(feature = "rand")]
fn test_rand() -> Result<(), String> {
    // Possible but incredibly small chance that this test could fail due to random sampling error.
    let abcd = TestByteFlags {
        TEST_A: 2,
        TEST_B: 1,
        TEST_C: 0,
        TEST_D: 0,
    };
    let mut v: Vec<TestByteFlags> = Vec::new();
    for _ in 0..100 {
        v.push(abcd.get_random());
    }
    assert!(v.contains(&TestByteFlags::TEST_A));
    assert!(v.contains(&TestByteFlags::TEST_B));
    assert!(!v.contains(&TestByteFlags::TEST_C));
    assert!(!v.contains(&TestByteFlags::TEST_D));
    assert!(
        v.iter().filter(|&x| *x == TestByteFlags::TEST_A).count()
            > v.iter().filter(|&x| *x == TestByteFlags::TEST_B).count()
    );
    Ok(())
}

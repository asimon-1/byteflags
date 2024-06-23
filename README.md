# byteflags

[![Rust](https://github.com/asimon-1/byteflags/workflows/Rust/badge.svg)](https://github.com/asimon-1/byteflags/actions)
[![Latest version](https://img.shields.io/crates/v/byteflags.svg)](https://crates.io/crates/byteflags)
[![Documentation](https://docs.rs/byteflags/badge.svg)](https://docs.rs/byteflags)
![License](https://img.shields.io/crates/l/byteflags.svg)

`byteflags` implements a [bitflags](https://docs.rs/bitflags/latest/bitflags/)-like structure using a full byte for each field instead of a bit. This allows the user to maintain the ergonomics of a bitflag structure while encoding additional information within.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
byteflags = "0.1.0"
```

and this to your source code:

```rust
use byteflags::*;
```

## Example

Generate a flags structure:

```rust
use byteflags::*;

byteflags! {
    /// The structure definition contains a set of named flags
    struct MyFlags {
        A = "Flag A",
        B = "Flag B",
        C = "Flag C",
        D = "Flag D",
    }
}

fn main() {
    /// The resulting structure can be used similarly to bitflags
    /// Constant values
    assert_eq!(
        MyFlags::A,
        MyFlags { A: 1, B: 0, C: 0, D: 0 }
    );
    /// Union
    assert_eq!(
        (MyFlags::A.union(MyFlags::B)),
        MyFlags { A: 1, B: 1, C: 0, D: 0 }
    );
    /// Difference
    assert_eq!(
        (MyFlags { A: 0, B: 1, C: 1, D: 1} - MyFlags {A: 0, B: 1, C: 0, D: 1 }),
        MyFlags { A: 0, B: 0, C: 1, D: 0}
    );

    /// Because each flag is a u8, you can do some arithmetic as well, including but not limited to:
    /// Addition
    assert_eq!(
        MyFlags::A + MyFlags::A + MyFlags::B,
        MyFlags { A: 2, B: 1, C: 0, D: 0 }
    );
    /// Multiplication by u8
    assert_eq!(
        (MyFlags { A: 2, B: 1, C: 0, D: 0 } * 3),
        MyFlags { A: 6, B: 3, C: 0, D: 0 }
    );
    /// Division by u8
    assert_eq!(
        (MyFlags { A: 60, B: 15, C: 0, D: 0 } / 3),
        MyFlags { A: 20, B: 5, C: 0, D: 0 }
    );
    /// Multiplication by other byteflag struct
    assert_eq!(
        (MyFlags { A: 1, B: 2, C: 3, D: 4 } * MyFlags { A: 0, B: 1, C: 2, D: 3 }),
        MyFlags { A: 0, B: 2, C: 6, D: 12 }
    );
    /// Subtractive Assignment
    let mut flags = MyFlags { A: 2, B: 2, C: 1, D: 0, };
    flags -= MyFlags::C;
    assert_eq!(
        flags,
        MyFlags { A: 2, B: 2, C: 0, D: 0 }
    );

    /// There are a couple other bells and whistles, such as:
    /// Conversion and serialization
    let flags = MyFlags { A: 0, B: 1, C: 10, D: 100 };
    assert_eq!(flags.to_vec(), vec![0, 1, 10, 100]);
    assert_eq!(serde_json::to_string(&flags).unwrap(), "[0,1,10,100]".to_string());
    assert_eq!(flags, serde_json::from_str::<MyFlags>("[0,1,10,100]").unwrap());

    /// contains() and match
    let d = MyFlags::D;
    assert_eq!(
        match d {
            MyFlags::A => "No",
            MyFlags::B => "Nope",
            MyFlags::C => "Wrong",
            MyFlags::D => "This one!",
            _ => "",
        },
        "This one!"
    );
    let flags = MyFlags { A: 0, B: 1, C: 10, D: 100 };
    assert!(flags.contains(&MyFlags::B));
    assert!(!flags.contains(&MyFlags::A));

    /// Display with labels
    assert_eq!(
        format!("{}", MyFlags::A),
        "Flag A"
    );
    assert_eq!(
        format!("{}", MyFlags::A + MyFlags::B),
        "Flag A + Flag B"
    );

    /// Access a flag via index
    assert_eq!(
        MyFlags::ALL_CONSTS[2],
        MyFlags::C
    );

    /// Get random flag with the "rand" feature
    /// Probabilities are weighted according to the flag value
    let weights = MyFlags{ A: 255, B: 127, C: 0, D: 0};
    let random_value = weights.get_random();
    // 66% chance of being MyFlags::A, 33% chance of being MyFlags::B
}

```

## Opinionated Design Choices

These design choices might not be immediately obvious. If you require a different behavior, please feel free to submit a pull request.

```rust
use byteflags::*;

byteflags! {
    struct MyFlags {
        A = "Flag A",
        B = "Flag B",
    }
}
fn main() {
    /// All arithmetic is saturating
    assert_eq!(
        (MyFlags{ A: 200, B: 0 } + MyFlags{ A: 200, B: 0 }),
        MyFlags{ A: u8::MAX, B: 0 }
    );
    assert_eq!(
        (MyFlags::A - (MyFlags::A + MyFlags::A)),
        MyFlags::empty()
    );
    assert_eq!(
        (MyFlags{ A: 128, B: 1 } * 128),
        MyFlags{ A: u8::MAX, B: 128 }
    );

    /// contains() only checks if a value is nonzero
    assert_eq!(
        (MyFlags::A).contains(&MyFlags{ A: 100, B: 0 }),
        true
    );

    /// Display also only checks if a value is nonzero
    assert_eq!(
        format!("{}", MyFlags{ A: 5, B: 10 }),
        "Flag A + Flag B"
    );

    /// all() gives values of 1
    assert_eq!(
        MyFlags::all(),
        MyFlags{ A: 1, B: 1 }
    );
}
```

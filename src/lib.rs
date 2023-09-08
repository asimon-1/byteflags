#![allow(non_snake_case, dead_code)]
pub mod __private {
    pub use core;
    #[cfg(feature = "rand")]
    pub use rand;
    pub use serde;
    pub use serde::ser::SerializeSeq;
    pub use std;
}

#[macro_export]
macro_rules! count {
    // From https://stackoverflow.com/a/34324856
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}
#[cfg(feature = "rand")]
pub fn get_random_int(max: usize) -> usize {
    use __private::rand::Rng;
    __private::rand::thread_rng().gen_range(0..max)
}

#[macro_export]
macro_rules! byteflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $ByteFlags:ident {
            $(
                $inner_vis:vis $Flag:ident -> $Name:literal,
            )*
        }
    ) => {
        $(#[$outer])*
         // Don't need to explicitly implement deserialize for some reason
        #[derive(PartialEq, Eq, $crate::__private::serde::Deserialize, Copy, Clone)]
        $vis struct $ByteFlags {
            $(
                $inner_vis $Flag : u8,
            )*
        }

        impl $crate::__private::serde::Serialize for $ByteFlags {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::__private::serde::Serializer,
            {
                use $crate::__private::SerializeSeq;
                let mut seq = serializer.serialize_seq(Some(count!($($Flag)*)))?;
                $(
                    seq.serialize_element(&self.$Flag)?;
                )*
                seq.end()
            }
        }

        impl $crate::__private::std::fmt::Display for $ByteFlags {
            fn fmt(&self, f: &mut $crate::__private::std::fmt::Formatter<'_>) -> $crate::__private::std::fmt::Result {
                let mut v: Vec<String> = Vec::new();
                $(
                    if self.contains(&Self::$Flag) {
                        v.push($Name.to_string());
                    }
                )*
                write!(f, "{}", v.join(" + "))
            }
        }

        impl $crate::__private::std::ops::Add<Self> for $ByteFlags {
            type Output = Self;
            fn add(self, rhs: $ByteFlags) -> Self {
                Self {
                    $(
                        $Flag: self.$Flag.saturating_add(rhs.$Flag),
                    )*
                }
            }
        }

        impl $crate::__private::std::ops::AddAssign<Self> for $ByteFlags {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl $crate::__private::std::ops::Sub<Self> for $ByteFlags {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self {
                    $(
                        $Flag: self.$Flag.saturating_sub(rhs.$Flag),
                    )*
                }
            }
        }

        impl $crate::__private::std::ops::SubAssign<Self> for $ByteFlags {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }


        impl $crate::__private::std::ops::Mul<u8> for $ByteFlags {
            type Output = Self;
            fn mul(self, rhs: u8) -> Self::Output {
                Self {
                    $($Flag: self.$Flag.saturating_mul(rhs),)*
                }
            }
        }

        impl $crate::__private::std::ops::MulAssign<u8> for $ByteFlags {
            fn mul_assign(&mut self, rhs: u8) {
                *self = *self * rhs;
            }
        }

        impl $crate::__private::std::ops::Mul<Self> for $ByteFlags {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self {
                    $($Flag: self.$Flag.saturating_mul(rhs.$Flag),)*
                }
            }
        }

        impl $crate::__private::std::ops::MulAssign<Self> for $ByteFlags {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl $crate::__private::std::ops::Div<u8> for $ByteFlags {
            type Output = Self;
            fn div(self, rhs: u8) -> Self::Output {
                Self {
                    $($Flag: self.$Flag.saturating_div(rhs),)*
                }
            }
        }

        impl $crate::__private::std::ops::Div<Self> for $ByteFlags {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                Self {
                    $($Flag: self.$Flag.saturating_div(rhs.$Flag),)*
                }
            }
        }

        impl $crate::__private::std::ops::DivAssign<u8> for $ByteFlags {
            fn div_assign(&mut self, rhs: u8) {
                *self = *self / rhs
            }
        }

        impl $crate::__private::std::ops::DivAssign<Self> for $ByteFlags {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs
            }
        }

        impl $ByteFlags {
            pub const fn new() -> Self {
                Self {
                    $(
                        $Flag: 0,
                    )*
                }
            }

            $(
                // Create enum-like associated consts
                // e.g. MyByteFlags::OPTION_A
                pub const $Flag: Self = Self {
                    $Flag: 1,
                    ..Self::new()
                };
            )*

            fn to_vec(&self) -> Vec<u8> {
                let mut vec = Vec::<u8>::new();
                $(vec.push(self.$Flag);)*
                vec
            }

            fn contains(&self, other: &Self) -> bool {
                // Only cares about zero / nonzero values.
                [
                    $(
                        (other.$Flag == 0) | ((self.$Flag > 0) && (other.$Flag > 0)),
                    )*

                ].iter().all(|x| *x)
            }

            fn union(&self, other: Self) -> Self {
                *self + other
            }

            /// Return the values from self which are nonzero in other
            fn left_intersection(&self, other: Self) -> Self {
                Self {
                    $($Flag: if self.$Flag > 0 && other.$Flag > 0 { self.$Flag } else { 0 }),*
                }
            }

            #[cfg(feature = "rand")]
            fn get_random(&self) -> Self {
                let mut v: Vec<Self> = Vec::new();
                if self == &Self::new() {
                    return Self::new();
                }
                $(
                    for _ in 0..self.$Flag {
                        v.push(Self::$Flag)
                    }
                )*
                if v.len() > 0 {
                    v[$crate::get_random_int(v.len())]
                } else { Self::new() }
            }
        }
    }
}

byteflags! {
    #[derive(Debug)]
    pub struct ExampleByteFlags {
        pub ALPHA -> "Alpha",
        pub BETA -> "Beta",
        pub CHARLIE -> "Charlie",
    }
}

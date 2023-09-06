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
                    if self.contains(&$ByteFlags::$Flag) {
                        v.push($Name.to_string());
                    }
                )*
                write!(f, "{}", v.join(" + "))
            }
        }

        impl $crate::__private::std::ops::Add<$ByteFlags> for $ByteFlags {
            type Output = $ByteFlags;
            fn add(self, rhs: $ByteFlags) -> $ByteFlags {
                $ByteFlags {
                    $(
                        $Flag: self.$Flag.checked_add(rhs.$Flag).unwrap_or(u8::MAX),
                    )*
                }
            }
        }

        impl $crate::__private::std::ops::AddAssign<$ByteFlags> for $ByteFlags {
            fn add_assign(&mut self, rhs: $ByteFlags) {
                *self = *self + rhs;
            }
        }

        impl $crate::__private::std::ops::Sub<$ByteFlags> for $ByteFlags {
            type Output = $ByteFlags;
            fn sub(self, rhs: $ByteFlags) -> $ByteFlags {
                $ByteFlags {
                    $(
                        $Flag: self.$Flag.checked_sub(rhs.$Flag).unwrap_or(u8::MIN),
                    )*
                }
            }
        }

        impl $crate::__private::std::ops::SubAssign<$ByteFlags> for $ByteFlags {
            fn sub_assign(&mut self, rhs: $ByteFlags) {
                *self = *self - rhs;
            }
        }


        impl $crate::__private::std::ops::Mul<u8> for $ByteFlags {
            type Output = $ByteFlags;
            fn mul(self, rhs: u8) -> Self::Output {
                $ByteFlags {
                    $($Flag: self.$Flag.checked_mul(rhs).unwrap_or(u8::MAX),)*
                }
            }
        }

        impl $crate::__private::std::ops::MulAssign<u8> for $ByteFlags {
            fn mul_assign(&mut self, rhs: u8) {
                *self = *self * rhs;
            }
        }

        impl $ByteFlags {
            const fn new() -> $ByteFlags {
                $ByteFlags {
                    $(
                        $Flag: 0,
                    )*
                }
            }

            $(
                // Create enum-like associated consts
                // e.g. MyByteFlags::OPTION_A
                const $Flag: $ByteFlags = $ByteFlags {
                    $Flag: 1,
                    ..$ByteFlags::new()
                };
            )*

            fn to_vec(&self) -> Vec<u8> {
                let mut vec = Vec::<u8>::new();
                $(vec.push(self.$Flag);)*
                vec
            }

            fn contains(&self, other: &$ByteFlags) -> bool {
                // Only cares about zero / nonzero values.
                [
                    $(
                        (other.$Flag == 0) | ((self.$Flag > 0) && (other.$Flag > 0)),
                    )*

                ].iter().all(|x| *x)
            }

            #[cfg(feature = "rand")]
            fn get_random(&self) -> $ByteFlags {
                let mut v: Vec<$ByteFlags> = Vec::new();
                if self == &$ByteFlags::new() {
                    return $ByteFlags::new();
                }
                $(
                    for _ in 0..self.$Flag {
                        v.push($ByteFlags::$Flag)
                    }
                )*
                if v.len() > 0 {
                    v[$crate::get_random_int(v.len())]
                } else { $ByteFlags::new() }
            }
        }
    }
}

#![allow(non_snake_case)]
// use rand::Rng;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::ops;

#[macro_export]
macro_rules! count {
    // From https://stackoverflow.com/a/34324856
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! multiselect {
    (
        $(#[$outer:meta])*
        $vis:vis struct $MultiSelect:ident {
            $(
                $inner_vis:vis $Flag:ident -> $Name:literal,
            )*
        }
    ) => {
        $(#[$outer])*
         // Don't need to explicitly implement deserialize for some reason
        #[derive(PartialEq, Eq, Deserialize, Copy, Clone)]
        $vis struct $MultiSelect {
            $(
                $inner_vis $Flag : u8,
            )*
        }

        impl Serialize for $MultiSelect {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(count!($($Flag)*)))?;
                $(
                    seq.serialize_element(&self.$Flag)?;
                )*
                seq.end()
            }
        }

        impl fmt::Display for $MultiSelect {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut v: Vec<String> = Vec::new();
                $(
                    if self.contains(&$MultiSelect::$Flag) {
                        v.push($Name.to_string());
                    }
                )*
                write!(f, "{}", v.join(" + "))
            }
        }

        impl ops::Add<$MultiSelect> for $MultiSelect {
            type Output = $MultiSelect;
            fn add(self, rhs: $MultiSelect) -> $MultiSelect {
                $MultiSelect {
                    $(
                        $Flag: self.$Flag.checked_add(rhs.$Flag).unwrap_or(u8::MAX),
                    )*
                }
            }
        }

        impl ops::Mul<u8> for $MultiSelect {
            type Output = $MultiSelect;
            fn mul(self, rhs: u8) -> Self::Output {
                $MultiSelect {
                    $($Flag: self.$Flag.checked_mul(rhs).unwrap_or(u8::MAX),)*
                }
            }
        }

        impl $MultiSelect {
            const fn new() -> $MultiSelect {
                $MultiSelect {
                    $(
                        $Flag: 0,
                    )*
                }
            }

            $(
                // Create enum-like associated consts
                // e.g. MyMultiSelect::OPTION_A
                const $Flag: $MultiSelect = $MultiSelect {
                    $Flag: 1,
                    ..$MultiSelect::new()
                };
            )*

            fn to_vec(&self) -> Vec<u8> {
                // TODO!(Do we need this?)
                let mut vec = Vec::<u8>::new();
                $(vec.push(self.$Flag);)*
                vec
            }

            fn contains(&self, other: &$MultiSelect) -> bool {
                // Only cares about zero / nonzero values.
                [
                    $(
                        (other.$Flag == 0) | ((self.$Flag > 0) && (other.$Flag > 0)),
                    )*

                ].iter().all(|x| *x)
            }

            // TODO!(Figure out how to make this compatible with smash feature branch)
            // fn get_random(&self) -> $MultiSelect {
            //     let mut v: Vec<$MultiSelect> = Vec::new();
            //     if self == &$MultiSelect::new() {
            //         return $MultiSelect::new();
            //     }
            //     $(
            //         for _ in 0..self.$Flag {
            //             v.push($MultiSelect::$Flag)
            //         }
            //     )*
            //     if v.len() > 0 {
            //         v[get_random_int(v.len())]
            //     } else { $MultiSelect::new() }
            // }
        }
    }
}

